use crate::client::Client;
use crate::connection::Connection;
use crate::connection::ConnectionStatus;
use crate::errors::ConnectionError;
use crate::protocol::command::Command;
use crate::protocol::reader::ReadProtocol;
use crate::protocol::writer::WriteProtocol;
use crate::selection::filter::FilterBuilder;
use futures::lock::Mutex;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

pub struct Eater {}

impl Eater {
    pub async fn read(connection: Arc<Connection>) -> Result<ConnectionStatus, ConnectionError> {
        let mut reader = connection.client.reader.lock().await;

        match reader.read_command().await {
            Ok(Command::Message) => {
                let id: Vec<u8> = reader.read_id().await;
                let size = reader.read_size().await;
                let (message, size) = reader.read_message().await;

                tokio::spawn(Eater::acknowledge(
                    Ok(()),
                    id,
                    connection.client.writer.clone(),
                ));
            }
            Ok(Command::Schema) => {
                /* Get message ID: */
                let mut id = [0u8; 8];
                reader.read_exact(&mut id).await.unwrap();

                /* Get message type: */
                let mut message_type = [0u8; 4];
                reader.read_exact(&mut message_type).await.unwrap();

                /* Get length: */
                let size = reader.read_size().await;

                /* Get message type: */
                let mut schema = vec![0u8; size as usize];
                reader.read_exact(&mut schema).await.unwrap();
                let mut registry = connection.router.registry.write().await;

                let raw_schema = String::from_utf8(schema).expect("Found invalid UTF-8");
                registry.add(message_type.to_vec(), raw_schema);
            }
            Ok(Command::SchemaNeeded) => {
                /* Get message type: */
                let mut message_type = [0u8; 4];
                reader.read_exact(&mut message_type).await.unwrap();

                let registry = connection.router.registry.read().await;

                let raw_schema = registry
                    .get_raw(message_type.to_vec())
                    .expect("Requested model that is not in registry");

                let mut writer = connection.client.writer.lock().await;

                /* Write command */
                writer.write_command(Command::Schema).await;

                /* Write message type: */
                writer
                    .write(&message_type[..])
                    .await
                    .expect("Cannot write message type");

                /* Write length: */
                writer
                    .write(&(raw_schema.len() as u32).to_le_bytes())
                    .await
                    .expect("Cannot write message length");

                /* Write message: */
                writer
                    .write(raw_schema.as_bytes())
                    .await
                    .expect("Cannot write message");
            }
            Ok(Command::Subscription) => {
                /* Get offset: */
                let mut raw_offset = [0u8; 8];
                reader.read_exact(&mut raw_offset).await.unwrap();
                let offset = u64::from_le_bytes(raw_offset);

                /* Get subscription ID: */
                let mut subscription_id = [0u8; 4];
                reader.read_exact(&mut subscription_id).await.unwrap();

                /* Get filter length: */
                let size = reader.read_size().await;

                /* Get filter: */
                let mut raw_filter = vec![0u8; size as usize];
                reader.read_exact(&mut raw_filter).await.unwrap();
                let filter: serde_json::Value = serde_json::from_str(
                    std::str::from_utf8(&raw_filter).expect("Cannot decode filter"),
                )
                .expect("Cannot parse filter");
                let store = connection.router.store.read().await;
                let reading_from_store = store.offset < offset;

                /* Add subscription: */
                let subscription =
                    Client::add_subscription(connection.client.clone(), reading_from_store, filter)
                        .await
                        .expect("Cannot add subscription");

                /* Integrate subscription: */
                {
                    let mut selector = connection.router.selector.write().await;
                    subscription.filter.integrate(&mut selector)
                }
            }
            _ => {
                return Ok(ConnectionStatus::End);
            }
        }
        Ok(ConnectionStatus::Keep)
    }
    pub async fn acknowledge(
        result: Result<(), ()>,
        id: Vec<u8>,
        writer: Arc<Mutex<OwnedWriteHalf>>,
    ) {
        match result {
            Ok(_) => {
                let mut writer = writer.lock().await;
                writer.write_message_ack(id.as_slice()).await;
            }
            Err(_) => {
                let mut writer = writer.lock().await;
                writer.write_message_nack(id.as_slice()).await;
            }
        }
    }
}

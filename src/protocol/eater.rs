use crate::client::Client;
use crate::connection::Connection;
use crate::connection::ConnectionStatus;
use crate::errors::ConnectionError;
use crate::protocol::command::Command;
use crate::protocol::reader::ReadProtocol;
use crate::protocol::writer::WriteProtocol;
use crate::selection::selector::{Selection};
use futures::lock::Mutex;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

pub struct Eater {}

impl Eater {
    pub async fn read(connection: Arc<Connection>) -> Result<ConnectionStatus, ConnectionError> {
        let mut reader = connection.client.reader.lock().await;

        let command = reader.read_command().await;

        match command {
            Ok(Command::Message) => {
                /* Get message ID: */
                let mut id = [0u8; 6];
                reader.read_exact(&mut id).await.unwrap();
                //println!("Message id: {:?}", id);

                /* Get message type: */
                let mut message_type = [0u8; 4];
                reader.read_exact(&mut message_type).await.unwrap();
                //println!("Message type: {:?}", message_type);

                /* Get message: */
                let (message, size) = reader.read_message().await;
                //println!("Message: {:?}", message);

                /* Access registry: */
                let registry = connection.router.registry.read().await;

                /* Get schema: */
                let schema = match registry.get(&message_type) {
                    Some(schema) => schema,
                    None => {
                        let mut writer = connection.client.writer.lock().await;
                        
                        /* Write command */
                        writer.write_command(Command::SchemaNeeded).await;

                        /* Write message type: */
                        writer
                            .write(&message_type[..])
                            .await
                            .expect("Cannot write message type");
                        /* Write command */
                        writer.write_command(Command::MessageNack).await;

                        /* Write message id: */
                        writer
                            .write(&id[..])
                            .await
                            .expect("Cannot write message id");

                        return Ok(ConnectionStatus::Keep);
                    }
                };

                /* Apply selector: */
                let selector = connection.router.selector.read().await;
                selector
                    .distribute(
                        &message_type[..],
                        &schema,
                        &message,
                    )
                    .await;
                //println!("Distribution completed");

                /* Write to storage: */
                {

                    //println!("Writing to storage");
                    let mut store = connection.router.store.write().await;
                    store.append(id, message_type, message).expect("Cannot write to storage");
                    //println!("Wrote to storage");
                }

                tokio::spawn(Eater::acknowledge(
                    Ok(()),
                    id.to_vec(),
                    connection.client.writer.clone(),
                ));
            }
            Ok(Command::Schema) => {
                /* Get message type: */
                let mut message_type = [0u8; 4];
                reader.read_exact(&mut message_type).await.unwrap();

                /* Get length: */
                let size = reader.read_size().await;
                //println!("Schema size is {}", size);

                /* Get schema: */
                let mut schema = vec![0u8; size as usize];
                reader.read_exact(&mut schema).await.unwrap();
                //println!("Schema is {:?}", schema);

                /* Access registry: */
                let mut registry = connection.router.registry.write().await;

                /* Decode and store schema: */
                let raw_schema = String::from_utf8(schema).expect("Found invalid UTF-8");
                registry.add(message_type.to_vec(), raw_schema);
            }
            Ok(Command::SchemaNeeded) => {
                /* Get message type: */
                let mut message_type = [0u8; 4];
                reader.read_exact(&mut message_type).await.unwrap();

                let registry = connection.router.registry.read().await;

                let mut writer = connection.client.writer.lock().await;

                let raw_schema = match registry.get_raw(message_type.to_vec()) {
                    Ok(raw_schema) => raw_schema,
                    Err(_) => {
                        /* Write command */
                        writer.write_command(Command::NoSchema).await;

                        /* Write message type: */
                        writer
                            .write(&message_type[..])
                            .await
                            .expect("Cannot write message type");

                        return Ok(ConnectionStatus::Keep);
                    }
                };

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
                reader.read_exact(&mut raw_offset[2..8]).await.unwrap();
                let offset = u64::from_le_bytes(raw_offset);
                //println!("Offset: {:?} -> {}", raw_offset, offset);

                /* Get subscription ID: */
                let mut subscription_id = [0u8; 4];
                reader.read_exact(&mut subscription_id).await.unwrap();
                //println!("Subscription id: {:?}", subscription_id);

                /* Get filter length: */
                let size = reader.read_size().await;
                //println!("Filter size: {}", size);

                /* Get filter: */
                let mut raw_filter = vec![0u8; size as usize];
                reader.read_exact(&mut raw_filter).await.unwrap();


                let filter: serde_json::Value = serde_json::from_str(
                    std::str::from_utf8(&raw_filter).expect("Cannot decode filter"),
                ).expect("Cannot deserialise filter");
                //println!("Filter: {:?} -> {:?}", raw_filter, filter);

                let store = connection.router.store.read().await;
                let reading_from_store = store.current > offset;

                /* Add subscription: */
                let subscription = Client::add_subscription(
                    connection.client.clone(),
                    subscription_id.to_vec(),
                    filter,
                )
                .await
                .expect("Cannot add subscription");
                //println!("Built subscription");

                /* Attach to store until all contents read: */
                {
                    let store = connection.router.store.read().await;
                    store.pipe(offset, subscription.clone(), connection.router.clone()).await;
                }

                /* Integrate subscription: */
                {
                    let mut selector = connection.router.selector.write().await;
                    subscription.integrate(&mut selector);
                    //println!("Integrated subscription");
                }
            },
            _ => {
                //println!("End connection");
                return Ok(ConnectionStatus::End);
            }
        }

        //println!("Keep connection");
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

use tokio::net::tcp::{OwnedWriteHalf};
use crate::errors::ConnectionError;
use crate::connection::Connection;
use tokio::io::AsyncReadExt;
use futures::lock::Mutex;
use std::sync::Arc;
use crate::connection::ConnectionStatus;
use crate::protocol::writer::WriteProtocol;
use crate::protocol::reader::ReadProtocol;
use crate::protocol::command::Command;

pub struct Eater {}

impl Eater {
    pub async fn read(connection: Arc<Connection>) -> Result<ConnectionStatus, ConnectionError> {
        let mut reader = connection.client.reader.lock().await;

        match reader.read_command().await {
            Command::Publish => {
                println!("Command: Publish");
                let id: Vec<u8> = reader.read_id().await;
                let size = reader.read_size().await;
                let mut message: Vec<u8> = Vec::with_capacity(size);
                unsafe { message.set_len(size); }

                reader.read_exact(&mut message).await.unwrap();

                println!("Message (id: {}, size: {}): {:?}", std::str::from_utf8(&id).unwrap(), size, message);
                
                println!("Spawning );
                tokio::spawn(Eater::acknowledge(Ok(()), id, connection.client.writer.clone()));
                println!("End of reading");

                ();
            },
            _ => {
                return Ok(ConnectionStatus::End);
            }
        }
        
        Ok(ConnectionStatus::Keep)
    }
    pub async fn acknowledge(result: Result<(), ()>, id: Vec<u8>, writer: Arc<Mutex<OwnedWriteHalf>>) {
        match result {
            Ok(_) => {
                let mut writer = writer.lock().await;
                writer.write_ack(id.as_slice()).await;
            }
            Err(_) => {
                let mut writer = writer.lock().await;
                writer.write_nack(id.as_slice()).await;
            }
        }
    }
}
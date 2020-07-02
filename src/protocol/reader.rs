use crate::protocol::command::Command;
use async_trait::async_trait;
use std::convert::TryFrom;
use std::io::Error;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;

#[async_trait]
pub trait ReadProtocol {
    async fn read_command(&mut self) -> Result<Command, Error>;
    async fn read_id(&mut self) -> Vec<u8>;
    async fn read_message(&mut self) -> (Vec<u8>, u32);
    async fn read_size(&mut self) -> u32;
}

#[async_trait]
impl ReadProtocol for OwnedReadHalf {
    async fn read_command(&mut self) -> Result<Command, Error> {
        let mut command: &mut [u8] = &mut [0u8; 1];
        self.read_exact(&mut command).await?;

        let parsed_command = Command::try_from(command[0]).unwrap();

        //println!("{:?} received", parsed_command);
        Ok(parsed_command)
    }

    async fn read_id(&mut self) -> Vec<u8> {
        let mut id: Vec<u8> = vec![0u8; 8];

        /* Read message id: */
        self.read_exact(&mut id).await.unwrap();
        id
    }

    async fn read_message(&mut self) -> (Vec<u8>, u32) {
        let length = self.read_size().await;
        //println!("Message size: {}", length);

        let mut message = vec![0u8; length as usize];

        /* Read message: */
        self.read_exact(&mut message).await.unwrap();

        (message, length)
    }

    async fn read_size(&mut self) -> u32 {
        let mut raw_length = [0u8; 4];
        self.read_exact(&mut raw_length).await.unwrap();
        let length = u32::from_le_bytes(raw_length);

        length
    }
}

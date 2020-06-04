use async_trait::async_trait;
use tokio::net::tcp::{OwnedReadHalf};
use tokio::io::AsyncReadExt;
use std::convert::TryFrom;
use crate::protocol::command::Command;
use crate::codec::numbers::VarIntDecoder;
use std::io::Error;

#[async_trait]
pub trait ReadProtocol {
    async fn read_command(&mut self) -> Result<Command, Error>;
    async fn read_id(&mut self) -> Vec<u8>;
    async fn read_size(&mut self) -> usize;
}

#[async_trait]
impl ReadProtocol for OwnedReadHalf {
    async fn read_command(&mut self) -> Result<Command, Error> {
        let mut command: &mut [u8] = &mut [0u8; 1];
        self.read_exact(&mut command).await?;

        Ok(Command::try_from(command[0]).unwrap())
    }

    async fn read_id(&mut self) -> Vec<u8> {
        let mut id: Vec<u8> = [0u8; 10].to_vec();
        /* Read message id: */
        self.read_exact(&mut id).await.unwrap();
        
        println!("{:?}", &id);
        
        id
    }

    async fn read_size(&mut self) -> usize {
        /* Read message id: */
        self.read_varint().await
    }
}
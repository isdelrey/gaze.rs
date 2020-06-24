use async_trait::async_trait;
use std::time::SystemTime;
use tokio::net::tcp::{OwnedWriteHalf};
use rand::{thread_rng};
use tokio::io::AsyncWriteExt;
use crate::protocol::command::Command;
use rand::RngCore;

#[async_trait]
pub trait WriteProtocol {
    async fn write_command(&mut self, command: Command);

    async fn publish(&mut self, content: &[u8]) -> Vec<u8>;
    async fn write_ack(&mut self, id: &[u8]);
    async fn write_nack(&mut self, id: &[u8]);
    async fn write_id(&mut self) -> Vec<u8>;
}

#[async_trait]
impl WriteProtocol for OwnedWriteHalf {
    async fn publish(&mut self, content: &[u8]) -> Vec<u8> {
        self.write_command(Command::Publish).await;
        let id = self.write_id().await;
        self.write(content).await.unwrap();

        id
    }

    async fn write_command(&mut self, command: Command) {
        self.write(&[command as u8]).await.unwrap();
    }

    async fn write_ack(&mut self, id: &[u8]) {
        self.write_command(Command::Ack).await;
        self.write(id).await.unwrap();
    }

    async fn write_nack(&mut self, id: &[u8]) {
        self.write_command(Command::Nack).await;
        self.write(id).await.unwrap();
    }

    async fn write_id(&mut self) -> Vec<u8> {
        let timestamp_as_u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
        let mut timestamp = timestamp_as_u64.to_le_bytes();

        {
            let mut rng = thread_rng();
            rng.fill_bytes(&mut timestamp[1..4]);
        }

        let last_byte_random = timestamp[1];
        let last_byte_mask = 0b1111_1100;
        timestamp[2] = last_byte_random | last_byte_mask;

        let id = &timestamp[2..8];

        self.write_command(Command::Publish).await;
        self.write(id).await.unwrap();
        
        Vec::from(id)
    }
}
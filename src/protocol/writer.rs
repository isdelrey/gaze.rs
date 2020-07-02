use crate::protocol::command::Command;
use async_trait::async_trait;
use rand::thread_rng;
use rand::RngCore;
use std::time::SystemTime;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

#[async_trait]
pub trait WriteProtocol {
    async fn write_command(&mut self, command: Command);

    async fn write_message_ack(&mut self, id: &[u8]);
    async fn write_message_nack(&mut self, id: &[u8]);
    async fn write_size(&mut self, size: usize);
    async fn write_id(&mut self) -> Vec<u8>;
}

#[async_trait]
impl WriteProtocol for OwnedWriteHalf {
    async fn write_command(&mut self, command: Command) {
        println!("Writing {:?}", command);
        self.write(&[command as u8]).await.unwrap();
    }

    async fn write_message_ack(&mut self, id: &[u8]) {
        self.write_command(Command::MessageAck).await;
        self.write(id).await.unwrap();
    }

    async fn write_message_nack(&mut self, id: &[u8]) {
        self.write_command(Command::MessageNack).await;
        self.write(id).await.unwrap();
    }

    async fn write_size(&mut self, size: usize) {
        self.write(&(size as u32).to_le_bytes()).await.unwrap();
    }


    async fn write_id(&mut self) -> Vec<u8> {
        let timestamp_as_u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let mut timestamp = timestamp_as_u64.to_le_bytes();

        {
            let mut rng = thread_rng();
            rng.fill_bytes(&mut timestamp[1..4]);
        }

        let last_byte_random = timestamp[1];
        let last_byte_mask = 0b1111_1100;
        timestamp[2] = last_byte_random | last_byte_mask;

        let id = &timestamp[2..8];

        self.write(id).await.unwrap();
        Vec::from(id)
    }
}

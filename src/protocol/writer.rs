use async_trait::async_trait;
use std::time::SystemTime;
use crate::protocol::time::SmallestReadableString;
use tokio::net::tcp::{OwnedWriteHalf};
use rand::{Rng, thread_rng};
use std::iter;
use tokio::io::AsyncWriteExt;
use rand::distributions::Alphanumeric;
use crate::protocol::command::Command;

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
        let random: String;
        {
            let mut rng = thread_rng();
            random = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) )
            .take(4).collect();
        }

        let mut ns = [0u8; 6];
        let ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().to_smallest_readable_string(&mut ns);

        self.write_command(Command::Publish).await;
        self.write(ns).await.unwrap();
        self.write(random.as_bytes()).await.unwrap();

        [ns, random.as_bytes()].concat()
    }
}
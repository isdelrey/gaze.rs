use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc};
use futures::lock::Mutex;
use std::marker::Unpin;
use crate::protocol::out;
use crate::errors::EatingError;
use crate::server::Client;


pub async fn ack(client: Client) {
    let mut message_id: [u8; 16] = [0u8; 16];
    {
        let mut reader = client.reader.lock().await;
        /* Read message id: */
        match reader.read_exact(&mut message_id[..]).await {
            Ok(_) => {},
            Err(_) => {}
        };
    }

    match eat(client.reader).await {
        Ok(_) => {
            let mut writer = client.writer.lock().await;
            writer.write(&out::ack(&message_id)).await.unwrap();
        }
        Err(_) => {
            let mut writer = client.writer.lock().await;
            writer.write(&out::nack(&message_id)).await.unwrap();
        }
    }
}

pub async fn eat<'a, R: AsyncReadExt + Unpin>(reader: Arc<Mutex<R>>) -> Result<&'a [u8], EatingError> {
   Ok(&[0,1])
}
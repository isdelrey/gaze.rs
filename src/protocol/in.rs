use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn ack<R: AsyncReadExt, W: AsyncWriteExt>(reader: Arc<Mutex<R>>, writer: Arc<Mutex<W>>) {
    /* Read message id: */
    let mut message_id = [0u8; 16];
    reader.read_exact(&mut message_id[..])?;

    match take(reader).await {
        Ok(_) => {
            let client = client.take();
            client.send(out::ack(message_id));
        }
        Err(_) => {
            let client = client.take();
            client.send(out::nack(message_id));
        }
    }
}

pub async fn take<R: AsyncReadExt, W: AsyncWriteExt>(reader: Arc<Mutex<R>>, writer: Arc<Mutex<W>>) -> Result<(&[u8]), Error> {
    let codec::read(reader)
}
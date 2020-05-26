use tokio::net::{TcpStream};
use tokio::io::{AsyncRead, AsyncBufReadExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::prelude::*;
use std::sync::{Arc, Weak};
use futures::lock::Mutex;
use crate::errors::{ReceiveStringError};
use crate::selection::Selection;


pub struct Client {
    pub id: String,
    pub reader: Arc<Mutex<OwnedReadHalf>>,
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub address: String,
    pub selection: Vec<Selection>
}


impl Client {
    pub fn new(id: String, stream: TcpStream) -> Client {
        let address = stream.peer_addr().unwrap().to_string();
        let (reader, writer) = stream.into_split();

        Client {
            id,
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
            address,
            selection: Vec::new()
        }
    }
    /*
    pub async fn receive_string(&self, separator: u8) -> std::result::Result<String, ReceiveStringError> {
        let mut content = Vec::with_capacity(1024);
        let mut stream = self.stream.lock().await;
        let nread = stream.read_until(separator, &mut content).await?;

        let nfirstchar = if content[0] == b'\n' {1} else {0};
        let content_string = String::from(str::from_utf8(&content[nfirstchar..nread - 1])?);
        println!("Received {} bytes: {:?} -> {}", nread, &content[nfirstchar..nread - 1], content_string);
        
        Ok(content_string)
    }
    pub async fn receive_exact(&self, separator: u8) -> std::result::Result<String, ReceiveStringError> {
        let mut content = Vec::with_capacity(1024);
        let mut stream = self.stream.lock().await;
        let nread = stream.read_until(separator, &mut content).await?;

        let nfirstchar = if content[0] == b'\n' {1} else {0};
        let content_string = String::from(str::from_utf8(&content[nfirstchar..nread - 1])?);
        println!("Received {} bytes: {:?} -> {}", nread, &content[nfirstchar..nread - 1], content_string);
        
        Ok(content_string)
    }
    pub async fn send_string(&self, content: &str, separator: Option<char>) -> std::result::Result<(), std::io::Error> {
        self.send(String::from(content).as_bytes(), separator.as_ref().map(|x| *x as u8)).await
    }
    pub async fn send(&self, content: &[u8], separator: Option<u8>) -> std::result::Result<(), std::io::Error> {
        let mut stream = self.writer.lock().await;

        stream.write(content).await?;
        if let Some(separator) = separator {
            stream.write(&[separator]).await?;
        }

        Ok(())
    }*/
}
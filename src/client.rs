use tokio::net::{TcpStream};
use tokio::prelude::*;
use std::str;
use std::sync::{Arc};
use futures::lock::Mutex;
use crate::errors::{ReceiveStringError};
use crate::selection::Selection;

pub struct Client {
    stream: Arc<Mutex<tokio::io::BufReader<tokio::net::TcpStream>>>,
    pub address: String,
    selection: Vec<Selection>
}


impl<'a> Client {
    pub fn new(stream: TcpStream) -> Result<Client, std::io::Error> {
        let address =  stream.peer_addr()?.to_string();
        Ok(Client {
            stream: Arc::new(Mutex::new(tokio::io::BufReader::new(stream))),
            address,
            selection: Vec::new()
        })
    }
    pub async fn receive_string(&self, separator: u8) -> std::result::Result<String, ReceiveStringError> {
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
        let mut stream = self.stream.lock().await;

        stream.write(content).await?;
        if let Some(separator) = separator {
            stream.write(&[separator]).await?;
        }

        Ok(())
    }
    pub async fn end(&self) -> std::result::Result<(), std::io::Error> {
        let mut stream = self.stream.lock().await;

        stream.shutdown().await
    }
}
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
}
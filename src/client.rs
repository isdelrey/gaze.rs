use tokio::net::{TcpStream};
use tokio::io::{AsyncRead, AsyncBufReadExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::prelude::*;
use std::sync::{Arc, Weak};
use futures::lock::Mutex;
use rand::{Rng, thread_rng, RngCore};
use std::iter;
use crate::errors::{ReceiveStringError};
use crate::selection::Selection;


pub struct Client {
    pub id: Vec<u8>,
    pub reader: Arc<Mutex<OwnedReadHalf>>,
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub address: String,
    pub selection: Vec<Selection>
}


impl Client {
    pub fn new(stream: TcpStream) -> Client {
        let address = stream.peer_addr().unwrap().to_string();
        let (reader, writer) = stream.into_split();

        let mut id = [0u8; 8];
        {
            rand::thread_rng().fill_bytes(&mut id);   
        }

        Client {
            id: id.to_vec(),
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
            address,
            selection: Vec::new()
        }
    }
}
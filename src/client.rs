use crate::errors::ReceiveStringError;
use crate::selection::filter::*;
use crate::selection::subscription::Subscription;
use futures::lock::Mutex;
use rand::{thread_rng, Rng, RngCore};
use std::iter;
use std::sync::{Arc, Weak};
use tokio::io::{AsyncBufReadExt, AsyncRead};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::RwLock;

pub struct Client {
    pub id: Vec<u8>,
    pub reader: Arc<Mutex<OwnedReadHalf>>,
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub address: String,
    pub subscriptions: Arc<RwLock<Vec<Subscription>>>,
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
            subscriptions: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub async fn add_subscription(
        client: Arc<Client>,
        reading_from_store: bool,
        original_filter: serde_json::Value,
    ) -> Result<Subscription, ()> {
        let filter = Filter::parse(original_filter).unwrap();

        /* Create subscription: */
        let subscription = Subscription::new(client.clone(), reading_from_store, filter);

        let mut self_subscriptions = client.subscriptions.write().await;
        self_subscriptions.push(subscription);

        Ok(subscription)
    }
}

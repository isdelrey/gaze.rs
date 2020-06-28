use crate::client::Client;
use crate::codec::registry::Registry;
use crate::selection::selector::Selector;
use crate::storage::Store;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use tokio::sync::RwLock;

pub struct Router {
    pub clients: RwLock<HashMap<Vec<u8>, Weak<Client>>>,
    pub selector: RwLock<Selector>,
    pub registry: RwLock<Registry>,
    pub store: RwLock<Store>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            clients: RwLock::new(HashMap::new()),
            selector: RwLock::new(Selector::new()),
            store: RwLock::new(Store::new()),
            registry: RwLock::new(Registry::new()),
        }
    }

    pub async fn add_client(&self, id: Vec<u8>, client: Arc<Client>) {
        let mut clients = self.clients.write().await;
        clients.insert(id, Arc::downgrade(&client));
    }

    pub async fn remove_client(&self, id: &Vec<u8>) {
        let mut clients = self.clients.write().await;
        let client = clients
            .remove(id)
            .unwrap()
            .upgrade()
            .expect("Client disconnection cannot not be dealt with");

        /* Disgregate client subcription filters from selector: */
        let subscriptions = client.subscriptions.read().await;
        let mut selector = self.selector.write().await;
        for subscription in subscriptions.iter() {
            subscription.disgregate(&mut selector);
        }
    }
}

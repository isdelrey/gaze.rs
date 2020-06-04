use tokio::io::{AsyncWriteExt};
use crate::client::{Client};
use std::sync::{Arc, Weak, RwLock};
use std::collections::HashMap;


pub struct Router {
    clients: RwLock<HashMap<String, Weak<Client>>>
}

impl Router {
    pub fn new() -> Router {
        Router {
            clients: RwLock::new(HashMap::new())
        }
    }

    pub fn add_client(&self, id: String, client: Arc<Client>) {
        let mut clients = self.clients.write().unwrap();
        clients.insert(id, Arc::downgrade(&client));
    }

    pub fn remove_client(&self, id: &str) {
        let mut clients = self.clients.write().unwrap();
        clients.remove(id);
    }

    pub async fn broadcast(&self, content: &[u8]) {
        let clients = self.clients.read().unwrap();
        for (id, client) in clients.iter() {
            let client = client.upgrade();
            match client {
                Some(client) => {
                    let mut writer = client.writer.lock().await;
                    writer.write(&content).await.unwrap();
                },
                None => {
                    self.remove_client(id);
                    continue;
                }
            };
        }
    }
}
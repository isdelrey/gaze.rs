use crate::client::Client;
use std::sync::Arc;

pub struct ServerRouter {
    pub clients: Vec<Arc<Client>>
}

impl ServerRouter {
    pub fn new() -> ServerRouter {
        ServerRouter {
            clients: vec![]
        }
    }

    pub fn add_client(&mut self, client: Arc<Client>) {
        self.clients.push(client);
    }

    pub async fn broadcast(&self, content: &[u8]) {
        for client in &self.clients {
            client.send(&content, Some(b'\n')).await;
        }
    }
    pub async fn broadcast_string(&self, content: &str) {
        for client in &self.clients {
            client.send_string(&content, Some('\n')).await;
        }
    }
}
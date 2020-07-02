use crate::client::Client;
use crate::protocol::eater::Eater;
use crate::router::Router;
use std::sync::Arc;

pub enum ConnectionStatus {
    Keep,
    End,
}

pub struct Connection {
    pub router: Arc<Router>,
    pub client: Arc<Client>,
}

impl Connection {
    pub fn new(router: Arc<Router>, client: Arc<Client>) -> Connection {
        Connection {
            router: router.clone(),
            client: client.clone(),
        }
    }

    pub async fn accept(connection: Connection) {
        //println!("Accepted connection from {}", connection.client.address);
        let connection = Arc::new(connection);

        /* Eating loop: */
        loop {
            match Eater::read(connection.clone()).await {
                Ok(ConnectionStatus::Keep) => {}
                Ok(ConnectionStatus::End) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}

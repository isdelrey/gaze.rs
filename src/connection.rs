use futures::lock::Mutex;
use std::sync::Arc;
use crate::router::Router;
use crate::client::Client;
use crate::errors::{ConnectionError};
use crate::protocol::eater::Eater;

pub enum ConnectionStatus {
    Keep,
    End
}

pub struct Connection {
    pub router: Arc<Mutex<Router>>,
    pub client: Arc<Client>
}


impl Connection {
    pub fn new(router: Arc<Mutex<Router>>, client: Arc<Client>) -> Connection {
        Connection {
            router: router.clone(),
            client: client.clone()
        }
    }

    pub async fn accept(connection: Connection) {
        /* Log: */
        println!("Accepted connection from {}", connection.client.address);
        let connection = Arc::new(connection);

        loop {
            match Eater::read(connection.clone()).await {
                Ok(ConnectionStatus::Keep) => {break;},
                Ok(ConnectionStatus::End) => {},
                Err(_) => {}
            }
            println!("End of message");
        }
    }
}
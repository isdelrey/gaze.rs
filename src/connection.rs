use futures::lock::Mutex;
use std::sync::Arc;
use crate::mode::{Mode};
use crate::server::{Server};
use crate::operation::Operation;
use crate::client::Client;
use crate::errors::{ConnectionError};

pub struct Connection {
    pub server: Arc<Mutex<Server>>,
    pub client: Arc<Client>
}


impl Connection {
    pub async fn accept(connection: Connection) -> Result<(), ConnectionError> {
        /* Log: */
        println!("Accepted connection from {}", connection.client.address);
        
        /* Initiate: */
        connection.client.send_string(format!("gaze {}", crate::VERSION).as_str(),Some('\r')).await?;
        
        /* Mode: */
        connection.client.send_string("Do you want to interact on string or byte mode (S/b)? ", None).await?;
        let mode = Mode::parse(connection.client.receive_string(b'\r').await?)?;
        println!("Got mode {:?} from {}", mode, connection.client.address);

        let connection = Arc::new(connection);

        loop {
            /* Operation: */
            connection.client.send_string("> ", None).await?;
            let block: String = connection.client.receive_string(b'\r').await?;

            let (operation, content) = Operation::parse(block)?;

            match operation {
                Operation::Close => {
                    println!("Client {} requested a disconnection", connection.client.address);
                    break
                },
                operation => {
                    println!("Got operation type {:?} from {}", operation, connection.client.address);
                    Operation::run(operation, connection.clone(), content).await;
                }
            }
        }

        Ok(())
    }
}
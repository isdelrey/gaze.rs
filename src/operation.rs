use crate::connection::{Connection};
use crate::errors::ParseError;
use std::sync::Arc;

#[derive(PartialEq, Debug)]
pub enum SchemaOperation {
    Add,
    Update
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Subscribe,
    Publish,
    Schema(SchemaOperation),
    Close
}


impl Operation {
    pub fn parse(block: String) -> Result<(Operation, String), ParseError> {
        let candidate = "subscribe";
        if block.starts_with("subscribe") {
            return Ok((Operation::Subscribe,String::from(&block[candidate.len() + 1..block.len()])))
        }
        let candidate = "publish";
        if block.starts_with("publish") {
            return Ok((Operation::Publish,String::from(&block[candidate.len() + 1..block.len()])))
        }
        let candidate = "exit";
        if block.starts_with(candidate) {
            return Ok((Operation::Close,String::from(&block[candidate.len() + 1..block.len()])))
        }

        let candidate = "schema add";
        if block.starts_with(candidate) {
            return Ok((Operation::Schema(SchemaOperation::Add),String::from(&block[candidate.len() + 1..block.len()])))
        }
        
        
        Err(ParseError)
    }
    pub async fn run(operation: Operation, connection: Arc<Connection>, content: String) {
        match operation {
            Operation::Subscribe => {
                Operation::subscribe(connection, content).await;
            },
            Operation::Schema(SchemaOperation::Add) => {
                Operation::subscribe(connection, content).await;
            },
            _ => {
                Operation::publish(connection, content).await;
            }
        }
    }

    async fn publish<'a>(connection: Arc<Connection>, message: String) {
        let server = connection.server.lock().await;
        server.router.broadcast_string(message.as_str()).await;
        println!("Got message {} from {}", message, connection.client.address)
    }

    async fn subscribe<'a>(connection: Arc<Connection>, content: String) {
        
        connection.client.send_string("Hello", Some('\n')).await;
    }
}
pub struct Message<'a> {
    content: &'a str
}
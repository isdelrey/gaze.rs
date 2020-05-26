use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::net::tcp::{OwnedReadHalf};
use crate::errors::ConnectionError;
use crate::server::Connection;
use std::sync::Arc;
use tokio::io::BufReader;
use crate::protocol::types::Command;
use crate::codec::number::*;

pub enum ConnectionStatus {
    Keep,
    End
}

pub async fn receive(connection: Arc<Connection>) -> Result<ConnectionStatus, ConnectionError> {
    let mut reader = connection.client.reader.lock().await;

    let command = &mut [1 as u8; 1];
    reader.read_exact(command).await.unwrap();
    println!("Command {:?}", command[0]);
    
    let (id, id_size): (i128, usize) = reader.read_varint_into_i128().await;
    println!("{} {}", id, id_size);

    /*
    loop {
        match reader.read_exact(buffer).await {
            Ok(_) => {},
            Err(error) => {
                println!("{:?}", error);
                break;
            }
        }
        if buffer[0] == Command::Next as u8 {
            break;
        }
        else {
            print!("{}", buffer[0] as char);
        }
    }*/

    
    Ok(ConnectionStatus::Keep)
}

/*
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

async fn publish<'a, R: AsyncWriteExt>(connection: Arc<Connection>, message: &[u8]) {
    let server = connection.server.lock().await;
    server.router.broadcast_string(message.as_str()).await;
    println!("Got message {} from {}", message, connection.client.address)
}

async fn subscribe<'a, >(connection: Arc<Connection>, content: String) {
    
    connection.client.send_string("Hello", Some('\n')).await;
}*/
use std::sync::{Arc};
use tokio::net::{TcpListener, TcpStream};
use crate::client::{Client};
use futures::stream::StreamExt;
use tokio::net::tcp::Incoming;
use crate::router::Router;
use futures::lock::Mutex;
use crate::mode::{Mode};
use crate::connection::Connection;

pub struct Server {
    pub router: Router
}

impl Server {
    pub fn new() -> Server {
        Server {
            router: Router::new()
        }
    }
    pub async fn accept_connection(server: Arc<Mutex<Server>>, stream: TcpStream) {
        let client = Arc::new(Client::new(stream).unwrap());
        {
            let server = server.clone();
            let mut server = server.lock().await;
            server.router.add_client(client.clone());
        }

        match Connection::accept(Connection {
            server: server.clone(),
            client: client.clone()
        }).await {
            Ok(()) => {
                println!("Client disconnected after successful communication");
                client.clone().end().await.expect("Client connection could not be terminated");
            },
            Err(error) => {
                println!("Client produced an {:?} error", error);
                client.clone().end().await.expect("Client connection could not be terminated");
            }
        };
    }
    pub async fn create() {
        let addr = "127.0.0.1:6142";
        let mut listener = TcpListener::bind(addr).await.unwrap();

        let server = Arc::new(Mutex::new(Server::new()));

        println!("Server running on localhost:6142");
        let mut incoming: Incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
           match stream {
                Ok(stream) => {
                    tokio::spawn(Server::accept_connection(server.clone(), stream));
                }
                Err(err) => {
                    println!("accept error = {:?}", err);
                }
            }
        }
    }
}
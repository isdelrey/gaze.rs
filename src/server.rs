use crate::client::Client;
use crate::connection::Connection;
use crate::router::Router;
use futures::stream::StreamExt;
use std::sync::Arc;
use tokio::net::tcp::Incoming;
use tokio::net::{TcpListener, TcpStream};

pub async fn handle_incoming(stream: TcpStream, router: Arc<Router>) {
    let client: Client = Client::new(stream);
    let client: Arc<Client> = Arc::new(client);

    router.add_client(client.id.clone(), client.clone()).await;

    let connection: Connection = Connection::new(router.clone(), client.clone());
    tokio::spawn(Connection::accept(connection));

    router.remove_client(&client.id).await;
}

pub async fn create<'c>() {
    let addr = "0.0.0.0:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let router: Arc<Router> = Arc::new(Router::new());

    println!("Ready on localhost:6142");
    let mut incoming: Incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(stream) => handle_incoming(stream, router.clone()).await,
            Err(err) => {
                println!("accept error = {:?}", err);
            }
        };
    }
}

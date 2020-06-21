use std::sync::{Arc};
use tokio::net::{TcpListener, TcpStream};
use futures::stream::StreamExt;
use tokio::net::tcp::Incoming;
use futures::lock::Mutex;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use crate::router::Router;
use crate::connection::Connection;
use crate::client::Client;
use uuid::Builder;

pub async fn handle_incoming(stream: TcpStream, router: Arc<Mutex<Router>>) {
        /* Get remote ip and build client id as a UUID: */
        let remote_ip = &stream.peer_addr().unwrap().to_string().into_bytes() as &[u8];
        let client_id_builder = Builder::from_slice(remote_ip);

        let client: Client = Client::new(stream);
        let client: Arc<Client> = Arc::new(client);

        {
            let router = router.clone();
            let router = router.lock().await;
            router.add_client(client.id.clone(), client.clone());
        }

        let connection: Connection = Connection::new(router.clone(), client.clone());
        tokio::spawn(Connection::accept(connection));

        {
            let router = router.clone();
            let router = router.lock().await;
            router.remove_client(&client.id);
        }
}

pub async fn create<'c>() {
    let addr = "0.0.0.0:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let router: Arc<Mutex<Router>> = Arc::new(Mutex::new(Router::new()));

    loop {
        let mut router = router.lock().await;
        let filler = vec![8u8; 30];
        router.store.append(&filler).unwrap();
    }

    println!("Ready on localhost:6142");
    let mut incoming: Incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(stream) => {
                handle_incoming(stream, router.clone()).await
             }
            Err(err) => {
                println!("accept error = {:?}", err);
            }
        };

    }
}
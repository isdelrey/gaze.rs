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

pub async fn create<'c>() {
    let addr = "0.0.0.0:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let router: Arc<Mutex<Router>> = Arc::new(Mutex::new(Router::new()));

    println!("Server running on localhost:6142");
    let mut incoming: Incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream: TcpStream = match stream {
            Ok(stream) => { stream }
            Err(err) => {
                println!("accept error = {:?}", err);
                continue;
            }
        };

        /* Get remote ip and build client id as a UUID: */
        let remote_ip = &stream.peer_addr().unwrap().to_string().into_bytes() as &[u8];
        let client_id_builder = Builder::from_slice(remote_ip);
        let client_id = String::from("");

        let client: Client = Client::new(client_id, stream);
        let client: Arc<Client> = Arc::new(client);

        {
            let router = router.clone();
            let router = router.lock().await;
            router.add_client(client.id.clone(), client.clone());
        }

        let connection: Connection = Connection::new(router.clone(), client.clone());
        Connection::accept(connection).await;

        {
            let router = router.clone();
            let router = router.lock().await;
            router.remove_client(&client.id);
        }
    }
}
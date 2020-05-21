mod client;
mod connection;
mod errors;
mod mode;
mod operation;
mod registry;
mod router;
mod schema;
mod selection;
mod server;
mod subscription;
mod codec;


use std::thread;
use server::Server;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    tokio::spawn(Server::create());
    
    println!("Server started");
    thread::park();

    ()
}

mod errors;
mod selection;
mod server;
mod codec;
mod protocol;


use std::thread;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    tokio::spawn(server::create());
    
    println!("Server started");
    thread::park();

    ()
}

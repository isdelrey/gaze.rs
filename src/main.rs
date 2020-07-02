#![feature(map_first_last)]

mod errors;
mod selection;
mod server;
mod codec;
mod protocol;
mod connection;
mod client;
mod router;
mod storage;


use std::thread;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    tokio::spawn(server::create());
    
    //println!("Gaze started");
    thread::park();

    ()
}

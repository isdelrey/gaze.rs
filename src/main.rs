#![feature(map_first_last)]

mod client;
mod codec;
mod connection;
mod errors;
mod protocol;
mod router;
mod selection;
mod server;
mod storage;

use std::thread;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    tokio::spawn(server::create());
    println!("Gaze started");

    tokio::spawn(protocol::eater::Eater::report_total_messages());
    thread::park();

    ()
}

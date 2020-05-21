use std::net::TcpStream;
use std::error::Error;
use std::boxed::Box;
use fasthash::{metro, MetroHasher};

fn main() -> Result<(), Box<dyn Error>>  {
    println!("Started");

    let ip = if let ip = std::env::var("IP").unwrap() {ip} else {"10.0.18.214".to_string()};
    
    println!("Connecting to {}:6142", ip);
    let socket: TcpStream = match TcpStream::connect(format!("{}:6142", ip)) {
        Ok(socket) => socket,
        Err(error) => {
            println!("Cannot connect to server: {}", error);
            return Err(Box::new(error))
        }
    };

    println!("Connected to {}", socket.peer_addr().unwrap());

    socket.write(protocol::publish("hello"));

    let hash = metro::hash64(b"hello world\xff");

    socket.write(hash);

    Ok(())    
}
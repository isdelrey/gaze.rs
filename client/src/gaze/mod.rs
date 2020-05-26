mod protocol;
mod encode;

use std::error::Error;
use std::boxed::Box;
use std::net::TcpStream;
use std::io::Write;

pub struct Gaze {
    pub stream: TcpStream,
}

impl Gaze {
    pub fn connect() -> Result<Gaze, Box<dyn Error>> {
        let ip = if let ip = std::env::var("IP").unwrap() {ip} else {"10.0.18.214".to_string()};
        println!("About to connect to Gaze on {}:6142", ip);

        let stream: TcpStream = match TcpStream::connect(format!("{}:6142", ip)) {
            Ok(stream) => {
                println!("Connected to Gaze on {}:6142", ip);
                stream
            },
            Err(error) => {
                println!("Cannot connect to server: {}", error);
                return Err(Box::new(error))
            }
        };

        Ok(Gaze {
            stream
        })
    }
    pub fn publish(&mut self) -> Result<(), Box<dyn Error>> {
        let mut id = [1 as u8; 19];
        let id = protocol::generate_id(&mut id);
        self.stream.write(&protocol::publish(id, "content".as_bytes())).unwrap();
        Ok(())
    }
}
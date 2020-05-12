use tokio::net::{TcpStream};
use tokio::prelude::*;
use std::str;
use std::sync::{Arc};
use futures::lock::Mutex;
use crate::errors::{ReceiveStringError};
use crate::selection::Selection;

pub struct Subscription {
    subscriber: Arc<Mutex<tokio::io::BufReader<tokio::net::TcpStream>>>,
    selection: Vec<Selection>
}


impl<'a> Subscription {
    
}
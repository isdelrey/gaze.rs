extern crate derive_more;
use derive_more::{From};

use std::error::*;
use std::fmt;

#[derive(Debug)]
pub struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Cannot parse input")
    }
}

#[derive(Debug, From)]
pub enum ConnectionError {
    IO(std::io::Error),
    Parse(ParseError),
    Receive(ReceiveStringError)
}

#[derive(Debug, From)]
pub enum ReceiveStringError {
    IO(std::io::Error),
    Utf8(std::str::Utf8Error)
}
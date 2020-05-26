use crate::protocol::types::*;

pub fn publish(id: &[u8], content: &[u8]) -> Vec<u8> {
    [&[Command::Publish as u8], id, content].concat().to_vec()
}

pub fn ack(id: &[u8]) -> Vec<u8> {
    [&[Command::Subscribe as u8], id].concat().to_vec()
}

pub fn nack(id: &[u8]) -> Vec<u8> {
    [&[Command::Subscribe as u8], id].concat().to_vec()
}
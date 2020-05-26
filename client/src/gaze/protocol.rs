use std::time::SystemTime;
use crate::gaze::encode::{IntoI128, IntoU128, VarInt};
use rand::prelude::*;

pub enum Command {
    Publish = 0x07, /* UTF8: BELL */
    Subscribe = 0x05, /* UTF8: ENQUIRY */
    Ack = 0x06, /* UTF8: ACK */
    Nack = 0x15 /* UTF8: NACK */
}

pub fn publish(id: &[u8], content: &[u8]) -> Vec<u8> {
    [&[Command::Publish as u8], id, content].concat().to_vec()
}

pub fn ack(id: &[u8]) -> Vec<u8> {
    [&[Command::Subscribe as u8], id].concat().to_vec()
}

pub fn nack(id: &[u8]) -> Vec<u8> {
    [&[Command::Subscribe as u8], id].concat().to_vec()
}

pub fn generate_id(dst: &mut [u8]) -> &mut [u8] {
    let random = thread_rng().gen::<i128>();
    let ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i128;

    (ns + random as i128).encode_var(dst);
    println!("Timestamp {}, Random {}\nGenerated ID {}\n{:?}", ns, random, (ns + random as i128), dst);

    dst
}
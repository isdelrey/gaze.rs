use types::*;

pub fn publish(id: &[u8], content: &[u8]) {
    &[Command::Publish, id, content].join()
}

pub fn ack(id: &[u8]) {
    &[Command::Subscribe, id].join()
}

pub fn nack(id: &[u8]) {
    &[Command::Subscribe, id].join()
}
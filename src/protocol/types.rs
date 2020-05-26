#[derive(Debug)]
#[repr(u8)]
pub enum Command {
    Publish = 0x07, /* UTF8: BELL */
    Subscribe = 0x05, /* UTF8: ENQUIRY */
    Ack = 0x06, /* UTF8: ACK */
    Nack = 0x15, /* UTF8: NACK */
    Next = 0x04, /* UTF8: END OF TRANSMISSION */
    End = 0x19 /* UTF8: END OF MEDIUM */
}
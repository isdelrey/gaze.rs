use async_trait::async_trait;
use std::io::Read;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;

const MSB: u8 = 0b1000_0000;
const DROP_MSB: u8 = 0b0111_1111;

pub trait ZigZagIntoUnsigned: Sized + Copy {
    fn zigzag(self) -> usize;
}

pub trait ZigZagIntoSigned: Sized + Copy {
    fn zigzag(self) -> isize;
}

impl ZigZagIntoUnsigned for isize {
    fn zigzag(self) -> usize {
        ((self << 1) ^ (self >> 63)) as usize
    }
}

impl ZigZagIntoSigned for usize {
    fn zigzag(self) -> isize {
        ((self >> 1) ^ (-((self & 1) as isize)) as usize) as isize
    }
}

pub trait VarIntEncoder {
    fn varint_size(self) -> usize;
    fn create_varint_vec(self) -> Vec<u8>;
    fn encode_as_varint(self) -> Vec<u8>;
}

pub trait VarIntDecoder {
    fn read_varint(&self) -> Result<u64, ()>;
    fn read_varint_size(&self) -> Result<(usize, usize), ()>;
}

impl VarIntEncoder for usize {
    fn varint_size(mut self) -> usize {
        if self == 0 {
            return 1;
        }

        let mut size = 0;
        while self > 0 {
            size += 1;
            self >>= 7;
        }
        size
    }
    fn create_varint_vec(self) -> Vec<u8> {
        let size = self.varint_size();
        Vec::with_capacity(size)
    }

    fn encode_as_varint(self) -> Vec<u8> {
        let mut dst = self.create_varint_vec();
        let mut n = self;
        let mut i = 0;

        while n >= 0x80 {
            dst.push(MSB | (n as u8));
            i += 1;
            n >>= 7;
        }

        dst.push(n as u8);
        dst
    }
}

impl VarIntDecoder for &[u8] {
    fn read_varint(&self) -> Result<u64, ()> {
        let mut result: u64 = 0;
        let mut shift = 0;

        for i in 0..self.len() {
            let msb_dropped = self[i] & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if self[i] & MSB == 0 {
                break;
            }
        }

        Ok(result)
    }
    fn read_varint_size(&self) -> Result<(usize, usize), ()> {
        let mut result: usize = 0;
        let mut shift = 0;
        let mut i = 0;

        while i < self.len() {
            i = i + 1;
            let msb_dropped = self[i] & DROP_MSB;
            result |= (msb_dropped as usize) << shift;
            shift += 7;

            if self[i] & MSB == 0 {
                break;
            }
        }

        Ok((result, i))
    }
}

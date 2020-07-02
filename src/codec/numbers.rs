use async_trait::async_trait;
use std::io::Read;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;

const MSB: u8 = 0b1000_0000;
const DROP_MSB: u8 = 0b0111_1111;

pub trait ZigZagIntoUnsigned<R>: Sized + Copy {
    fn zigzag(self) -> R;
}

pub trait ZigZagIntoSigned<R>: Sized + Copy {
    fn zigzag(self) -> R;
}

impl ZigZagIntoUnsigned<usize> for isize {
    fn zigzag(self) -> usize {
        ((self << 1) ^ (self >> 63)) as usize
    }
}

impl ZigZagIntoUnsigned<u64> for i64 {
    fn zigzag(self) -> u64 {
        ((self << 1) ^ (self >> 63)) as u64
    }
}

impl ZigZagIntoSigned<isize> for usize {
    fn zigzag(self) -> isize {
        ((self >> 1) ^ (-((self & 1) as isize)) as usize) as isize
    }
}
impl ZigZagIntoSigned<i64> for u64 {
    fn zigzag(self) -> i64 {
        ((self >> 1) ^ (-((self & 1) as i64)) as u64) as i64
    }
}

pub trait VarIntEncoder {
    fn varint_size(self) -> usize;
    fn encode_as_varint(self) -> Vec<u8>;
}

pub trait VarIntDecoder {
    fn get_varint_size(&self) -> Result<usize, ()>;
    fn read_varint_with_size(&self, length: usize) -> Result<u64, ()>;
    fn read_varint(&self) -> Result<(usize, usize), ()>;
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

    fn encode_as_varint(self) -> Vec<u8> {
        let mut dst = Vec::new();
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

impl VarIntEncoder for u64 {
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

    fn encode_as_varint(self) -> Vec<u8> {
        let mut dst = Vec::new();
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
    fn get_varint_size(&self) -> Result<usize, ()> {
        let mut i = 0;

        while i < self.len() {
            if self[i] & MSB == 0 {
                break;
            }

            i = i + 1;
        }

        Ok(i + 1)
    }
    fn read_varint_with_size(&self, length: usize) -> Result<u64, ()> {
        let mut result: u64 = 0;
        let mut shift = 0;

        for i in 0..length {
            let msb_dropped = self[i] & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;
        }

        Ok(result.zigzag() as u64)
    }
    fn read_varint(&self) -> Result<(usize, usize), ()> {
        let mut result: usize = 0;
        let mut shift = 0;
        let mut i = 0;

        while i < self.len() {
            //println!("v8: {:?}", self[i]);
            let msb_dropped = self[i] & DROP_MSB;
            result |= (msb_dropped as usize) << shift;
            shift += 7;

            if self[i] & MSB == 0 {
                break;
            }

            i = i + 1;
        }

        Ok((result.zigzag() as usize, i + 1))
    }
}

use async_trait::async_trait;
use tokio::io::{AsyncReadExt};

pub const MSB: u8 = 0b1000_0000;
const DROP_MSB: u8 = 0b0111_1111;

pub trait IntoU128: Sized + Copy {
    fn zigzag_into_u128(self) -> u128;
}

pub trait IntoI128: Sized + Copy {
    fn zigzag_into_i128(self) -> i128;
}

impl IntoU128 for i128 {
    fn zigzag_into_u128(self) -> u128 {
        ((self << 1) ^ (self >> 63)) as u128
    }
}

impl IntoI128 for u128 {
    fn zigzag_into_i128(self) -> i128 {
        ((self >> 1) ^ (-((self & 1) as i128)) as u128) as i128
    }
}

pub trait VarInt: Sized + Copy {
    fn required_space(self) -> usize;
    /// Decode a value from the slice. Returns the value and the number of bytes read from the
    /// slice (can be used to read several consecutive values from a big slice)
    fn decode_var(src: &[u8]) -> (Self, usize);
    /// Encode a value into the slice. The slice must be at least `required_space()` bytes long.
    /// The number of bytes taken by the encoded integer is returned.
    fn encode_var(self, src: &mut [u8]) -> usize;

    /// Helper: Encode a value and return the encoded form as Vec. The Vec must be at least
    /// `required_space()` bytes long.
    fn encode_var_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.resize(self.required_space(), 0);
        self.encode_var(&mut v);
        v
    }
}

impl VarInt for i128 {
    fn required_space(mut self) -> usize {
        if self == 0 {
            return 1;
        }

        let mut logcounter = 0;
        while self > 0 {
            logcounter += 1;
            self >>= 7;
        }
        logcounter
    }

    fn decode_var(src: &[u8]) -> (Self, usize) {
        let mut result: u128 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u128) << shift;
            shift += 7;

            if b & MSB == 0 || shift > (10 * 7) {
                break;
            }
        }

        (result.zigzag_into_i128() as Self, shift / 7 as usize)
    }

    #[inline]
    fn encode_var(self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= self.required_space());
        let mut n = (self as i128).zigzag_into_u128();
        let mut i = 0;

        while n >= 0x80 {
            dst[i] = MSB | (n as u8);
            i += 1;
            n >>= 7;
        }

        dst[i] = n as u8;
        i+1
    }
}

#[async_trait]
pub trait VarIntReader {
    async fn read_varint_into_i128(&mut self) -> (i128, usize);
}

#[async_trait]
impl<R: AsyncReadExt + Unpin + Sync + Send> VarIntReader for R {
    async fn read_varint_into_i128(&mut self) -> (i128, usize) {
        let mut result: u128 = 0;
        let mut shift = 0;

        let b: &mut [u8] = &mut [1 as u8; 1];
        while let Ok(_) = self.read_exact( b).await {
            println!("{:?}", b[0]);
            let msb_dropped = b[0] & DROP_MSB;
            result |= (msb_dropped as u128) << shift;
            shift += 7;

            if b[0] & MSB == 0 {
                println!("end {:?}", b[0] & MSB == 0);
                break;
            }
        }

        (result.zigzag_into_i128(), shift / 7 as usize)
    }
}
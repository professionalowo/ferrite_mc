use crate::server::connection::protocol::reader::{CONTINUE_BIT, SEGMENT_BITS};

use super::{MString, VarInt};

pub trait Serialize {
    fn serialize(&self) -> std::io::Result<Vec<u8>>;
}

impl Serialize for VarInt {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut value = self.0;
        let mut buf: Vec<u8> = Vec::new();

        loop {
            if value & !(SEGMENT_BITS) as i32 == 0 {
                buf.push(self.0 as u8);
                return Ok(buf);
            }
            let byte: u8 = match ((value & SEGMENT_BITS as i32) | CONTINUE_BIT as i32).try_into() {
                Ok(u) => u,
                Err(error) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
                }
            };
            buf.push(byte);
            value >>= 7;
        }
    }
}

impl Serialize for MString {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let size: usize = match self.size.0.try_into() {
            Ok(u) => u,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
        };

        let mut buf: Vec<u8> = Vec::with_capacity(size + 4);
        buf.append(&mut self.size.serialize()?);
        let mut self_bytes: Vec<u8> = self.value.bytes().into_iter().collect();
        buf.append(&mut self_bytes);
        Ok(buf)
    }
}

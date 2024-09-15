use crate::server::connection::protocol::reader::{CONTINUE_BIT, SEGMENT_BITS};

use super::{MString, VarInt, VarLong};

pub trait Serialize {
    fn serialize(&self) -> std::io::Result<Vec<u8>>;
}

impl Serialize for VarInt {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut value = self.0;
        let mut buf = Vec::new();

        loop {
            let byte = (value & SEGMENT_BITS as i32) as u8;
            if (value & !(SEGMENT_BITS as i32)) == 0 {
                // Last byte
                buf.push(byte);
                return Ok(buf);
            }

            // Push byte with continuation bit
            buf.push(byte | CONTINUE_BIT as u8);

            // Shift value to the right by 7 bits
            value >>= 7;
        }
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        Ok(self.to_be_bytes().to_vec())
    }
}

impl Serialize for VarLong {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut value = self.0;
        let mut buf = Vec::new();

        loop {
            let byte = (value & SEGMENT_BITS as i64) as u8;
            if (value & !(SEGMENT_BITS as i64)) == 0 {
                // Last byte
                buf.push(byte);
                return Ok(buf);
            }

            // Push byte with continuation bit
            buf.push(byte | CONTINUE_BIT as u8);

            // Shift value to the right by 7 bits
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

#[cfg(test)]
mod tests {
    use crate::server::connection::protocol::types::{serialize::Serialize, VarInt};

    #[test]
    fn serialize_one_byte() {
        let v = 127;
        let varint = VarInt(v);
        assert_eq!(varint.serialize().unwrap(), vec![127])
    }

    #[test]
    fn serialize_more_bytes() {
        let v = 255;
        let varint = VarInt(v);
        assert_eq!(varint.serialize().unwrap(), vec![0xFF, 0x1])
    }

    #[test]
    fn serialize_two_bytes() {
        let v = 300;
        let varint = VarInt(v);
        assert_eq!(varint.serialize().unwrap(), vec![0xAC, 0x02]);
    }
    #[test]
    fn serialize_three_bytes() {
        let v = 16_384;
        let varint = VarInt(v);
        assert_eq!(varint.serialize().unwrap(), vec![0x80, 0x80, 0x01]);
    }
}

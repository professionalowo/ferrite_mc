use std::io::{Cursor, Read};

use super::{
    types::{MString, VarInt, VarLong, UUID},
    Package,
};

pub const SEGMENT_BITS: u8 = 0x7F;
pub const CONTINUE_BIT: u8 = 0x80;
#[derive(Debug)]
pub struct ProtocolReader<R: Read>(pub R);

impl<R: Read> Read for ProtocolReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

pub trait ReadProtocol {
    fn try_read_package(&mut self) -> std::io::Result<ProtocolReader<Package>>;
    fn try_read_var_int(&mut self) -> std::io::Result<VarInt>;
    fn try_read_var_long(&mut self) -> std::io::Result<VarLong>;
    fn try_read_ushort(&mut self) -> std::io::Result<u16>;
    fn try_read_string(&mut self) -> std::io::Result<MString>;
    fn try_read_uuid(&mut self) -> std::io::Result<UUID>;
}

impl<R: Read> ReadProtocol for ProtocolReader<R> {
    fn try_read_package(&mut self) -> std::io::Result<ProtocolReader<Package>> {
        let length = self.try_read_var_int()?.value;
        let packet_id = self.try_read_var_int()?.value;

        let bytes_to_read: usize = match (length - packet_id).try_into() {
            Ok(u) => u,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        };

        let mut buf: Vec<u8> = vec![0; bytes_to_read + 1];
        self.read_exact(&mut buf)?;

        Ok(ProtocolReader(Package {
            length,
            id: packet_id,
            data: Cursor::new(buf),
        }))
    }

    fn try_read_var_int(&mut self) -> std::io::Result<VarInt> {
        let mut value: i32 = 0;
        let mut position: u8 = 0;
        let mut current_byte: [u8; 1] = [0];

        loop {
            self.read_exact(&mut current_byte)?;
            value |= (current_byte[0] as i32 & SEGMENT_BITS as i32) << position;

            if current_byte[0] & CONTINUE_BIT == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt too long",
                ));
            }
        }

        Ok(VarInt {
            value,
            length: (position / 7),
        })
    }

    fn try_read_var_long(&mut self) -> std::io::Result<VarLong> {
        let mut value: i64 = 0;
        let mut position: u8 = 0;
        let mut current_byte: [u8; 1] = [0];

        loop {
            self.read_exact(&mut current_byte)?;
            value |= (current_byte[0] as i64 & SEGMENT_BITS as i64) << position;

            if current_byte[0] & CONTINUE_BIT == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt too long",
                ));
            }
        }

        Ok(VarLong {
            value,
            length: (position / 7),
        })
    }

    fn try_read_ushort(&mut self) -> std::io::Result<u16> {
        let mut buf: [u8; 2] = [0, 0];
        self.read_exact(&mut buf)?;

        Ok(u16::from_be_bytes(buf))
    }

    fn try_read_string(&mut self) -> std::io::Result<MString> {
        let size = self.try_read_var_int()?;

        let bytes_to_read = match size.value.try_into() {
            Ok(u) => u,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
        };
        let mut buf: Vec<u8> = vec![0; bytes_to_read];
        self.read_exact(&mut buf)?;

        let value = match String::from_utf8(buf) {
            Ok(s) => s,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
        };

        Ok(MString { size, value })
    }

    fn try_read_uuid(&mut self) -> std::io::Result<UUID> {
        let mut buf: [u8; 16] = [0; 16];
        self.read_exact(&mut buf)?;

        Ok(UUID(u128::from_le_bytes(buf)))
    }
}

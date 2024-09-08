use std::io::{Cursor, Read};

use types::{serialize::Serialize, VarInt};

use super::Connection;
pub mod reader;
pub mod types;

pub struct ProtocolError(String);

#[derive(Debug, Clone)]
pub struct Package {
    pub length: i32,
    pub id: i32,
    data: Cursor<Vec<u8>>,
}

impl Read for Package {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(buf)
    }
}

impl Serialize for Package {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut length_bytes = VarInt(self.length).serialize()?;
        let mut id_bytes = VarInt(self.id).serialize()?;
        let mut data: Vec<u8> = self.data.clone().into_inner();

        let mut out = Vec::with_capacity(length_bytes.len() + id_bytes.len() + data.len());

        out.append(&mut length_bytes);
        out.append(&mut id_bytes);
        out.append(&mut data);

        Ok(out)
    }
}

impl Package {
    pub fn try_new<I>(id: i32, data: I) -> std::io::Result<Self>
    where
        I: IntoIterator<Item = u8>,
    {
        let id_bytes = VarInt(id).serialize()?;
        let data_bytes: Vec<u8> = data.into_iter().collect();

        let length = id_bytes.len() as i32 + data_bytes.len() as i32;

        Ok(Self {
            length,
            id,
            data: Cursor::new(data_bytes),
        })
    }
}

pub trait WriteProtocol {}

impl WriteProtocol for Connection {}

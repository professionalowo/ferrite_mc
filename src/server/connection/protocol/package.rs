use std::io::{Cursor, Read};

use super::types::{serialize::Serialize, VarInt};

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
    pub(super) fn new(id: i32, length: i32, data: Vec<u8>) -> Self {
        Self {
            length,
            id,
            data: Cursor::new(data),
        }
    }

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

    pub fn from_serializiable_iter<S, I>(id: i32, data: I) -> std::io::Result<Self>
    where
        S: Serialize,
        I: IntoIterator<Item = S>,
    {
        let data = data
            .into_iter()
            .flat_map(|s| s.serialize())
            .flatten()
            .collect::<Vec<_>>();

        Self::try_new(id, data)
    }

    pub fn from_serializiable<S: Serialize>(id: i32, data: S) -> std::io::Result<Self> {
        Self::try_new(id, data.serialize()?)
    }
}

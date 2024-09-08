use std::io::{Cursor, Read};

use types::VarInt;

use super::Connection;
pub mod reader;
pub mod types;

pub struct ProtocolError(String);

#[derive(Debug, Clone)]
pub struct Package {
    length: i32,
    id: i32,
    data: Cursor<Vec<u8>>,
}

impl Read for Package {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(buf)
    }
}

pub trait WriteProtocol {}

impl WriteProtocol for Connection {}

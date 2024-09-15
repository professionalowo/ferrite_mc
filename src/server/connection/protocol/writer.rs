use std::io::Write;

use crate::server::connection::Connection;

use super::{package::Package, types::serialize::Serialize};

pub trait WriteProtocol {
    fn write_packet(&mut self, packet: Package) -> std::io::Result<()>;
}

impl WriteProtocol for Connection {
    fn write_packet(&mut self, packet: Package) -> std::io::Result<()> {
        self.get_stream_mut().write_all(&packet.serialize()?[..])
    }
}

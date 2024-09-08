use connection_state::{test_response, ConnectionState};
use protocol::{
    reader::{ProtocolReader, ReadProtocol},
    types::{serialize::Serialize, MString, VarInt},
    Package,
};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod connection_state;
mod protocol;

#[derive(Debug)]
pub struct Connection {
    pub inner: ProtocolReader<TcpStream>,
    state: ConnectionState,
}

impl Connection {
    pub const fn new(stream: TcpStream) -> Self {
        Self {
            inner: ProtocolReader(stream),
            state: ConnectionState::Uninitialized,
        }
    }

    pub const fn get_state(&self) -> ConnectionState {
        self.state
    }

    pub fn write_package(&mut self, package: Package) -> std::io::Result<()> {
        match self.inner.0.write(&package.serialize()?) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn try_send_status(&mut self) -> std::io::Result<()> {
        let status_string = match serde_json::to_string(&test_response()) {
            Ok(s) => s,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
        };
        let m_string = MString {
            size: VarInt(status_string.len().try_into().unwrap()),
            value: status_string,
        };
        let out_bytes = m_string.serialize()?;
        let out_package = Package::try_new(0x00, out_bytes)?;
        self.write_package(out_package)
    }

    pub fn try_recieve_ping(&mut self) -> std::io::Result<()> {
        let mut pack = self.inner.try_read_package()?;
        let long = pack.try_read_long()?;
        println!("{:?}", pack);
        Ok(())
    }

    pub fn try_recieve_status_request(&mut self) -> std::io::Result<bool> {
        let pack = self.inner.try_read_package()?;
        Ok(pack.0.id == 0 && pack.0.length == 1)
    }

    pub fn try_handshake(&mut self) -> std::io::Result<()> {
        self.state = ConnectionState::Handshaking;
        let mut package = self.inner.try_read_package()?;
        let protocol_version = package.try_read_var_int()?.0;
        let server_address = package.try_read_string()?.value;
        let server_port = package.try_read_ushort()?;
        let next_state = package.try_read_var_int()?.0;

        println!("protocol_version: {:?}", protocol_version);
        println!("server_address: {:?}", server_address);
        println!("server_port: {:?}", server_port);
        println!("next_state: {:?}", next_state);
        self.state = next_state.try_into()?;
        Ok(())
    }
    pub fn try_login(&mut self) -> std::io::Result<()> {
        self.state = ConnectionState::Login;
        let mut package = self.inner.try_read_package()?;
        let name = package.try_read_string()?.value;
        let uuid = package.try_read_uuid()?;
        println!("name: {:?}", name);
        println!("uuid: {:X?}", uuid);
        Ok(())
    }

    pub fn try_set_encryption(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    pub fn try_set_compression(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    pub fn try_finish_login(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
//delegate read and write to underlying TcpStream
impl Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for Connection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.0.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::server::connection::protocol::reader::{ProtocolReader, ReadProtocol};

    use super::protocol::{
        types::{serialize::Serialize, MString, VarInt},
        Package,
    };

    #[test]
    fn serialization_roundtrip() {
        let value = "Hallo Welt";
        let mstring = MString {
            value: value.into(),
            size: VarInt(value.len().try_into().unwrap()),
        };
        let package = Package::try_new(0, mstring.serialize().unwrap()).unwrap();
        let bytes = package.serialize().unwrap();
        let cursor = Cursor::new(bytes);
        let mut reader = ProtocolReader(cursor);
        let mut p = reader.try_read_package().unwrap();
        let string = p.try_read_string().unwrap();
        assert_eq!(string.value, value)
    }
}

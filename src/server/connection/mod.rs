use connection_state::{test_response, ConnectionState};
use protocol::{
    reader::{ProtocolReader, ReadProtocol},
    types::{serialize::Serialize, MString, VarInt},
};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

mod connection_state;
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

    pub fn try_handshake(&mut self) -> std::io::Result<()> {
        self.state = ConnectionState::Handshaking;
        let mut package = self.inner.try_read_package()?;
        let protocol_version = package.try_read_var_int()?.value;
        let server_address = package.try_read_string()?.value;
        let server_port = package.try_read_ushort()?;
        let next_state = package.try_read_var_int()?.value;

        println!("protocol_version: {:?}", protocol_version);
        println!("server_address: {:?}", server_address);
        println!("server_port: {:?}", server_port);
        println!("next_state: {:?}", next_state);

        let status_response = serde_json::to_string(&test_response())?;

        self.inner.0.write(
            &MString {
                size: VarInt {
                    length: 1,
                    value: 1,
                },
                value: status_response,
            }
            .serialize()?,
        )?;

        Ok(())
    }
    pub fn try_login(&mut self) -> std::io::Result<()> {
        self.state = ConnectionState::Login;
        let mut package = self.inner.try_read_package()?;
        let name = package.try_read_string()?.value;
        let uuid = package.try_read_uuid()?;
        println!("name: {:?}", name);
        println!("uuid: {:?}", uuid);
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

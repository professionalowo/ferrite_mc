use connection_state::ConnectionState;
use protocol::reader::{ProtocolReader, ReadProtocol};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

mod connection_state;
mod protocol;

#[derive(Debug)]
pub struct Connection {
    inner: ProtocolReader<TcpStream>,
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
        let mut package = self.inner.try_read_package()?;
        let protocol_version = package.try_read_var_int()?.value;
        let server_address = package.try_read_string()?;
        let server_port = package.try_read_ushort()?;
        let next_state = package.try_read_var_int()?.value;

        println!("protocol_version: {:?}", protocol_version);
        println!("server_address: {:?}", server_address);
        println!("server_port: {:?}", server_port);
        println!("next_state: {:?}", next_state);

        self.state = ConnectionState::Handshaking;
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

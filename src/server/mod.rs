use std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    thread::{self, JoinHandle},
    time::Duration,
};

use connection::Connection;
mod connection;

pub struct Server<A>
where
    A: ToSocketAddrs,
{
    stream_handles: Vec<JoinHandle<std::io::Result<()>>>,
    address: A,
}

impl<A: ToSocketAddrs + std::marker::Send + 'static> Server<A> {
    pub const fn new(address: A) -> Self {
        Self {
            stream_handles: Vec::new(),
            address,
        }
    }

    pub fn run(mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address)?;
        for stream in listener.incoming().flatten() {
            let handle = thread::spawn(move || Self::handle_client(stream));
            self.stream_handles.push(handle);
        }

        for handle in self.stream_handles {
            handle.join().ok();
        }
        Ok(())
    }

    fn handle_client(stream: TcpStream) -> std::io::Result<()> {
        println!("Accepting connection from {}", stream.local_addr()?);
        let mut s = Connection::new(Self::configure_stream(&stream)?);
        s.try_handshake()?;
        s.try_login()?;
        s.try_set_encryption()?;
        s.try_set_compression()?;
        s.try_finish_login()?;

        Ok(())
    }

    fn configure_stream(stream: &TcpStream) -> std::io::Result<TcpStream> {
        let read_timeout_duration = Duration::from_secs(30);
        let write_timeout_duration = Duration::from_secs(1);

        let s = stream.try_clone()?;
        s.set_read_timeout(Some(read_timeout_duration))?;
        s.set_write_timeout(Some(write_timeout_duration))?;

        s.set_nodelay(false)?;
        s.set_nonblocking(false)?;
        Ok(s)
    }
}

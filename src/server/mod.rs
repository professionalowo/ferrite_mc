use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    thread::{self, JoinHandle},
    time::Duration,
};

pub struct Server<A>
where
    A: ToSocketAddrs,
{
    stream_handles: Vec<JoinHandle<std::io::Result<()>>>,
    address: A,
}

impl<A: ToSocketAddrs + std::marker::Send + 'static> Server<A> {
    pub fn new(address: A) -> Self {
        Self {
            stream_handles: Vec::new(),
            address,
        }
    }

    pub fn run(mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address)?;
        for stream in listener.incoming().flatten() {
            let handle = thread::spawn(|| Self::handle_client(stream));
            self.stream_handles.push(handle);
        }

        for handle in self.stream_handles {
            handle.join().ok();
        }
        Ok(())
    }

    fn handle_client(stream: TcpStream) -> std::io::Result<()> {
        println!("Accepting connection from {}", &stream.local_addr()?.ip());
        let s = Self::configure_stream(&stream)?;
        drop(stream);

        let mut reader = BufReader::new(s);
        let mut buf: Vec<u8> = Vec::with_capacity(250);
        while let Ok(n) = reader.read_until(20, &mut buf) {
            //is only reached after stream closes
            if n == 0 {
                break;
            }
        }
        println!("{:?}", buf);
        Ok(())
    }

    fn configure_stream(stream: &TcpStream) -> std::io::Result<TcpStream> {
        let read_timeout_duration = Duration::from_secs(5);
        let write_timeout_duration = Duration::from_secs(1);

        let s = stream.try_clone()?;
        s.set_read_timeout(Some(read_timeout_duration))?;
        s.set_write_timeout(Some(write_timeout_duration))?;

        s.set_nodelay(false)?;
        s.set_nonblocking(false)?;
        Ok(s)
    }
}

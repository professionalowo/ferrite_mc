use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};

fn configure_stream(stream: &TcpStream) -> std::io::Result<()> {
    let duration = Some(Duration::from_secs(2));

    stream.set_read_timeout(duration)?;
    stream.set_write_timeout(duration)?;

    stream.set_nodelay(true)?;
    stream.set_nonblocking(false)?;
    Ok(())
}

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    println!("Accepting connection from {}", stream.local_addr()?.ip());

    configure_stream(&stream)?;

    let mut reader = BufReader::new(stream);
    let mut buf: Vec<u8> = Vec::with_capacity(250);
    while let Ok(n) = reader.read_until(20, &mut buf) {
        if n == 0 {
            break;
        }
    }
    println!("{:?}", buf);
    Ok(())
}

pub struct Server {
    stream_handles: Vec<JoinHandle<std::io::Result<()>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            stream_handles: Vec::new(),
        }
    }

    pub fn run(mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:25565")?;
        for stream in listener.incoming().flatten() {
            let handle = thread::spawn(|| handle_client(stream));
            self.stream_handles.push(handle);
        }

        for handle in self.stream_handles {
            handle.join().ok();
        }
        Ok(())
    }
}

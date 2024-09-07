use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time::Duration,
};

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        print!("{}", line);
        line.clear();
        if bytes_read == 0 {
            break;
        }
    }

    Ok(())
}
fn main() -> std::io::Result<()> {
    let mut stream_handles: Vec<JoinHandle<std::io::Result<()>>> = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:25565")?;
    for stream in listener.incoming().flatten() {
        let handle = thread::spawn(|| handle_client(stream));
        stream_handles.push(handle);
    }

    for handle in stream_handles {
        handle.join().ok();
    }
    Ok(())
}

use server::Server;
pub mod server;

fn main() -> std::io::Result<()> {
    let server = Server::new();
    server.run()
}

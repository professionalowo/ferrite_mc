use std::net::Ipv4Addr;

use server::Server;
pub mod server;

fn main() -> std::io::Result<()> {
    let server = Server::new((Ipv4Addr::new(127, 0, 0, 1), 25565));
    server.run()
}

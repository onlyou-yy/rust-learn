mod handler;
mod router;
mod server;

use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:3000");
    server.run();
}

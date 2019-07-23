mod game;

pub fn main() {
  let server = lotsa::server::Server::new();
  server.start();
}

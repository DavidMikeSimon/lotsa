use std::io::Write;

use actix::{Actor, StreamHandler};
use actix_web::{fs, server, ws, App, HttpRequest, HttpResponse};
use bincode::serialize;
use flate2::{write::ZlibEncoder, Compression};

use crate::{chunk::Chunk, life::LIFE, point::Point};

struct LotsaWebsocketActor;

impl Actor for LotsaWebsocketActor {
  type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for LotsaWebsocketActor {
  fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
    let mut c = Chunk::new();
    c.set_block_type(Point::new(0, 0, 0), LIFE);
    c.set_block_type(Point::new(1, 1, 0), LIFE);
    c.set_block_type(Point::new(2, 2, 0), LIFE);
    c.set_block_type(Point::new(3, 3, 0), LIFE);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&serialize(&c).unwrap()).unwrap();
    let bytes = encoder.finish().unwrap();

    match msg {
      ws::Message::Ping(msg) => ctx.pong(&msg),
      ws::Message::Text(_text) => ctx.binary(bytes),
      _ => (),
    }
  }
}

fn webpack_dist() -> fs::StaticFiles<()> {
  // TODO: Don't rely on fixed directory name
  // TODO: Serve js directly from pkg folder, so we don't need symlink
  fs::StaticFiles::new("www")
    .expect("find www directory")
    .index_file("index.html")
}

fn websocket_handler(req: &HttpRequest<()>) -> Result<HttpResponse, actix_web::error::Error> {
  ws::start(req, LotsaWebsocketActor)
}

pub fn start() {
  server::new(|| {
    App::new()
      .resource("/ws/", |r| r.f(|req| websocket_handler(req)))
      .handler("/", webpack_dist())
  })
  .bind("127.0.0.1:8088")
  .expect("bind to open port") // TODO
  .run()
}

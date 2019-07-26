use std::io::Write;

use actix::*;
use actix::prelude::*;
use actix_files as fs;
use actix_web::{web, HttpRequest};
use actix_web_actors::ws;
use bincode::serialize;
use flate2::{write::ZlibEncoder, Compression};

use crate::{block::EMPTY, chunk::Chunk, debug::Debugger, life, sim::Simulator};

struct WebsocketSession {

}

impl WebsocketSession {
  fn new() -> WebsocketSession {
    WebsocketSession { }
  }
}

impl Actor for WebsocketSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    info!("ws session started");
  }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WebsocketSession {
  fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
    info!("got message {:?}", msg);

    let mut chunk = Chunk::new();
    chunk.fill_with_block_type(EMPTY);

    let debugger = Debugger::new(hashmap!(EMPTY => '.', life::LIFE => 'L'));
    debugger.load(
      &mut chunk,
      ".....
        .LLL.
        .....
        .....
        .....
        .LLL.
        .....",
    );

    // let mut sim = Simulator::new();
    // life::init(&mut sim);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&serialize(&chunk).expect("serialize chunk"))
        .expect("compress message");
    let bytes = encoder.finish().expect("finish compressing message");
    ctx.binary(bytes);
  }
}

type HttpResult = Result<actix_web::HttpResponse, actix_web::Error>;

fn websockets_route(req: HttpRequest, stream: web::Payload) -> HttpResult {
  ws::start(
    WebsocketSession::new(),
    &req,
    stream
  )
}

pub struct Server {
}

impl Server {
  pub fn new() -> Server {
    Server {}
  }

  pub fn start(&self) -> std::io::Result<()> {
    if let Err(_) = std::env::var("RUST_LOG") {
      std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let sys = System::new("lotsa");

    actix_web::HttpServer::new(move || {
      actix_web::App::new()
        .service(web::resource("/ws/").to(websockets_route))
        .service(fs::Files::new("/pkg/", "pkg/"))
        .service(fs::Files::new("/", "www/").index_file("index.html"))
    })
    .bind("127.0.0.1:8088")?
    .start();

    sys.run()
  }
}

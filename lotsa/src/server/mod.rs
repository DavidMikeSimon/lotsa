use std::io::Write;

use actix::*;
use actix::prelude::*;
use actix_files as fs;
use actix_web::{web, HttpRequest};
use actix_web_actors::ws;
use bincode::serialize;
use flate2::{write::ZlibEncoder, Compression};

use crate::{block::EMPTY, chunk::Chunk, debug::Debugger, life, sim::Simulator};

#[derive(Debug)]
struct ClientMessage { }

impl Message for ClientMessage {
  type Result = Vec<u8>;
}

struct World {
  chunk: Chunk,
  sim: Simulator
}

impl World {
  fn new() -> World {
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

    let mut sim = Simulator::new();
    life::init(&mut sim);

    World { chunk, sim }
  }
}

impl Actor for World {
  type Context = Context<Self>;
}

impl Handler<ClientMessage> for World {
  type Result = MessageResult<ClientMessage>;

  fn handle(&mut self, msg: ClientMessage, _ctx: &mut Context<Self>) -> Self::Result {
    info!("got client message {:?}", msg);
    let serialized = serialize(&self.chunk).expect("serialize chunk");

    self.sim.step(&mut self.chunk);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&serialized).expect("compress message");
    MessageResult(encoder.finish().expect("finish compressing message"))
  }
}

struct WebsocketSession { web_common: WebCommon }

impl WebsocketSession {
  fn new(web_common: WebCommon) -> WebsocketSession {
    WebsocketSession { web_common }
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
    info!("got ws message {:?}", msg);
    self.web_common.world
      .send(ClientMessage{})
      .into_actor(self)
      .then(|res, _, ctx| {
        match res {
          Ok(bytes) => ctx.binary(bytes),
          _ => ctx.stop(),
        }
        fut::ok(())
      })
      .wait(ctx);
  }
}

type HttpResult = Result<actix_web::HttpResponse, actix_web::Error>;

fn websockets_route(req: HttpRequest, stream: web::Payload, data: web::Data<WebCommon>) -> HttpResult {
  ws::start(
    WebsocketSession::new(data.get_ref().clone()),
    &req,
    stream
  )
}

#[derive(Clone)]
struct WebCommon {
  world: Addr<World>
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

    let world = World::new().start();

    actix_web::HttpServer::new(move || {
      actix_web::App::new()
        .data(WebCommon { world: world.clone() })
        .service(web::resource("/ws/").to(websockets_route))
        .service(fs::Files::new("/pkg/", "pkg/"))
        .service(fs::Files::new("/", "www/").index_file("index.html"))
    })
    .bind("127.0.0.1:8088")?
    .start();

    sys.run()
  }
}

use std::{collections::HashMap, io::Write, time::Duration};

use actix::prelude::*;
use actix::*;
use actix_files as fs;
use actix_web::{web, HttpRequest};
use actix_web_actors::ws;
use bincode::serialize;
use flate2::{write::ZlibEncoder, Compression};

use crate::{block::EMPTY, chunk::Chunk, debug::Debugger, life, sim::Simulator};

#[derive(Debug, Message)]
struct ClientMessage {}

#[derive(Debug, Message)]
struct ServerMessage {
  bytes: Vec<u8>
}

type SessionId = usize;

struct ClientConnected {
  session: Addr<WebsocketSession>
}

impl Message for ClientConnected {
  type Result = SessionId;
}

#[derive(Debug, Message)]
struct Tick {}

struct World {
  chunk: Chunk,
  sim: Simulator,
  next_id: usize,
  sessions: HashMap<usize, Addr<WebsocketSession>>
}

impl World {
  fn new() -> World {
    let mut chunk = Chunk::new();
    chunk.fill_with_block_type(EMPTY);

    Debugger::new(hashmap!(EMPTY => '.', life::LIFE => 'L')).load(
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

    World { chunk, sim, next_id: 1, sessions: HashMap::new() }
  }

  fn encode_chunk_and_step(&mut self) -> Vec<u8> {
    let serialized = serialize(&self.chunk).expect("serialize chunk");

    self.sim.step(&mut self.chunk);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&serialized).expect("compress message");
    encoder.finish().expect("finish compressing message")
  }
}

impl Actor for World {
  type Context = Context<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    ctx.run_interval(Duration::from_millis(100), |_, ctx| {
      ctx.address().do_send(Tick {});
    });
  }
}

impl Handler<ClientMessage> for World {
  type Result = ();

  fn handle(&mut self, msg: ClientMessage, _ctx: &mut Context<Self>) {
    info!("got client message {:?}", msg);
  }
}

impl Handler<ClientConnected> for World {
  type Result = usize;

  fn handle(&mut self, msg: ClientConnected, _ctx: &mut Context<Self>) -> Self::Result {
    let id = self.next_id;
    self.next_id = self.next_id + 1;
    info!("client #{} connected", id);
    self.sessions.insert(id, msg.session);
    id
  }
}

impl Handler<Tick> for World {
  type Result = ();

  fn handle(&mut self, _msg: Tick, ctx: &mut Context<Self>) {
    let bytes = self.encode_chunk_and_step();
    
    for (_id, session) in self.sessions.iter() {
      // FIXME: Probably inefficient to clone the vec
      session.try_send(ServerMessage { bytes: bytes.clone() }).expect("send message to client session");
    }
  }
}

struct WebsocketSession {
  id: Option<usize>,
  web_common: WebCommon,
}

impl WebsocketSession {
  fn new(web_common: WebCommon) -> WebsocketSession {
    WebsocketSession { id: None, web_common }
  }
}

impl Actor for WebsocketSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    info!("ws session started");
    self
      .web_common
      .world
      .send(ClientConnected { session: ctx.address() })
      .into_actor(self)
      .then(|res, act, ctx| {
        match res {
          Ok(id) => act.id = Some(id),
          _ => ctx.stop()
        }
        fut::ok(())
      })
      .wait(ctx);
  }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WebsocketSession {
  fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
    info!("got ws message {:?}", msg);
    self
      .web_common
      .world
      .try_send(ClientMessage {})
      .expect("send message to world process");
  }
}

impl Handler<ServerMessage> for WebsocketSession {
  type Result = ();

  fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
    ctx.binary(msg.bytes);
  }
}

type HttpResult = Result<actix_web::HttpResponse, actix_web::Error>;

fn websockets_route(
  req: HttpRequest,
  stream: web::Payload,
  data: web::Data<WebCommon>,
) -> HttpResult {
  ws::start(WebsocketSession::new(data.get_ref().clone()), &req, stream)
}

#[derive(Clone)]
struct WebCommon {
  world: Addr<World>,
}

pub struct Server {}

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
        .data(WebCommon {
          world: world.clone(),
        })
        .service(web::resource("/ws/").to(websockets_route))
        .service(fs::Files::new("/pkg/", "pkg/"))
        .service(fs::Files::new("/", "www/").index_file("index.html"))
    })
    .bind("127.0.0.1:8088")?
    .start();

    sys.run()
  }
}

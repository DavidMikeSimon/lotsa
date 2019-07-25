use std::{io::Write, sync::Arc};

use bincode::serialize;
use flate2::{write::ZlibEncoder, Compression};
use futures::{future::Future, stream::Stream};
use warp::Filter;

use crate::{block::EMPTY, chunk::Chunk, debug::Debugger, life::LIFE};

struct WsHandler {}

impl WsHandler {
  fn msg_response(&self, _msg: &warp::ws::Message) -> warp::ws::Message {
    let mut c = Chunk::new();
    c.fill_with_block_type(EMPTY);

    let debugger = Debugger::new(hashmap!(EMPTY => '.', LIFE => 'L'));
    debugger.load(
      &mut c,
      ".....
       .LLL.
       .....
       .....
       .....
       .LLL.",
    );

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
      .write_all(&serialize(&c).expect("serialize chunk"))
      .expect("compress message");
    let bytes = encoder.finish().expect("finish compressing message");

    warp::ws::Message::binary(bytes)
  }
}

pub struct Server {
  ws_handler: Arc<WsHandler>,
}

impl Server {
  pub fn new() -> Server {
    Server {
      ws_handler: Arc::new(WsHandler {}),
    }
  }

  pub fn start(&self) {
    if let Err(_) = std::env::var("RUST_LOG") {
      std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let root_route = warp::fs::dir("www");
    let pkg_route = warp::path("pkg").and(warp::fs::dir("pkg"));

    let ws_handler = self.ws_handler.clone();
    let ws_handler_state_route = warp::any().map(move || ws_handler.clone());
    let ws_route = warp::path("ws")
      .and(warp::ws2())
      .and(ws_handler_state_route)
      .map(|ws: warp::ws::Ws2, ws_handler: Arc<WsHandler>| {
        ws.on_upgrade(move |websocket| {
          let (socket_tx, socket_rx) = websocket.split();
          socket_rx
            .map(move |msg| ws_handler.msg_response(&msg))
            .forward(socket_tx)
            .map(|_| ())
            .map_err(|e| {
              error!("websocket error: {:?}", e);
            })
        })
      });

    let routes = root_route.or(pkg_route).or(ws_route);
    warp::serve(routes).run(([0, 0, 0, 0], 8088));
  }
}

use std::io::Write;

use bincode::serialize;
use flate2::{write::ZlibEncoder, Compression};
use futures::future::Future;
use futures::stream::Stream;
use warp::Filter;

use crate::{chunk::Chunk, life::LIFE, point::Point};

fn ws_response(_msg: &warp::ws::Message) -> warp::ws::Message {
  let mut c = Chunk::new();
  c.set_block_type(Point::new(0, 0, 0), LIFE);
  c.set_block_type(Point::new(1, 1, 0), LIFE);
  c.set_block_type(Point::new(2, 2, 0), LIFE);
  c.set_block_type(Point::new(3, 3, 0), LIFE);

  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  encoder.write_all(&serialize(&c).expect("serialize chunk")).expect("compress message");
  let bytes = encoder.finish().expect("finish compressing message");

  warp::ws::Message::binary(bytes)
}

pub fn start() {
  info!("starting server");

  let root_route = warp::fs::dir("www");
  let pkg_route = warp::path("pkg").and(warp::fs::dir("pkg"));
  let ws_route = warp::path("ws").and(warp::ws2()).map(|ws: warp::ws::Ws2| {
    ws.on_upgrade(|websocket| {
      let (tx, rx) = websocket.split();
      rx
        .map(|msg| ws_response(&msg))
        .forward(tx)
        .map(|_| ())
        .map_err(|e| { error!("websocket error: {:?}", e); } )
    })
  });

  let routes = root_route.or(pkg_route).or(ws_route);
  warp::serve(routes).run(([0, 0, 0, 0], 8088));
}

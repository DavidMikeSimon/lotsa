use std::{io::Write, sync::atomic::{AtomicUsize, Ordering}, sync::Arc};

use bincode::serialize;
use flate2::{write::ZlibEncoder, Compression};
use futures::{future::Future, stream::Stream, sync::mpsc};
use futures_locks::{Mutex, RwLock};
use warp::Filter;
use tokio;

use crate::{block::EMPTY, chunk::Chunk, debug::Debugger, life, sim::Simulator};

struct WebsocketConn {
  id: usize,
  tx: mpsc::UnboundedSender<warp::ws::Message>,
}

struct ServerImpl {
  chunk: RwLock<Chunk>,
  sim: Simulator,
  next_conn_id: AtomicUsize,
  conns: Mutex<Vec<Arc<WebsocketConn>>>
}

impl ServerImpl {
  fn new() -> ServerImpl {
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

    ServerImpl {
      chunk: RwLock::new(chunk),
      sim,
      next_conn_id: AtomicUsize::new(1),
      conns: Mutex::new(Vec::new())
    }
  }

  fn accept_connection(
    self: Arc<ServerImpl>,
    websocket: warp::ws::WebSocket,
  ) -> impl Future<Item = (), Error = ()> {
    let (socket_tx, socket_rx) = websocket.split();
    let (tx, rx) = mpsc::unbounded();
    let id = self.next_conn_id.fetch_add(1, Ordering::Relaxed);
    let conn = Arc::new(WebsocketConn { tx, id });

    info!("got connection #{}", id);

    tokio::spawn(
      rx.map_err(|()| -> warp::Error { unreachable!("unbounded rx never fails") })
        .forward(socket_tx)
        .map(|_| ())
        .map_err(|e| error!("websocket send error: {:?}", e))
    );

    let self2 = self.clone();

    self.conns
      .lock()
      .map(move |conns| conns.push(conn.clone()))
      .and_then(move |_| {
        socket_rx
          .map_err(|e| {
            error!("websocket error: {:?}", e);
          })
          .for_each(move |msg| self2.received_msg(&conn, &msg))
          .then(move |_| self2.lost_connection(id))
      })
  }

  fn received_msg(&self, conn: &WebsocketConn, _msg: &warp::ws::Message) -> impl Future<Item = (), Error = ()> + '_ {
    self
      .send_chunk(conn)
      .and_then(|()| self.chunk.write())
      .map(|chunk| self.sim.step(&mut chunk))
      .map_err(|_| ())
  }

  fn send_chunk(&self, conn: &WebsocketConn) -> impl Future<Item = (), Error = ()> {
    self.chunk
      .read()
      .map(|chunk| {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
          .write_all(&serialize(&*chunk).expect("serialize chunk"))
          .expect("compress message");
        let bytes = encoder.finish().expect("finish compressing message");
        let msg = warp::ws::Message::binary(bytes);

        // If we can't send the message, e.g. if client disconnected, don't worry about it
        // TODO: Maybe track this as a stat
        let _ = conn.tx.unbounded_send(msg);
      })
  }

  fn lost_connection(self: Arc<ServerImpl>, id: usize) -> impl Future<Item = (), Error = ()> {
    info!("lost connection #{}", id);
    self.conns
      .lock()
      .map(|conns| {
        match conns.iter().position(|conn| conn.id == id) {
          Some(idx) => { conns.swap_remove(idx); () },
          None => (),
        };
      })
  }
}

pub struct Server {
  server_impl: Arc<ServerImpl>,
}

impl Server {
  pub fn new() -> Server {
    let server_impl = Arc::new(ServerImpl::new());

    Server { server_impl }
  }

  pub fn start(&self) {
    if let Err(_) = std::env::var("RUST_LOG") {
      std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let root_route = warp::fs::dir("www");
    let pkg_route = warp::path("pkg").and(warp::fs::dir("pkg"));

    let server_impl = self.server_impl.clone();
    let server_impl_state_route = warp::any().map(move || server_impl.clone());
    let ws_route = warp::path("ws")
      .and(warp::ws2())
      .and(server_impl_state_route)
      .map(|ws: warp::ws::Ws2, server_impl: Arc<ServerImpl>| {
        ws.on_upgrade(move |websocket| server_impl.accept_connection(websocket))
      });

    let routes = root_route.or(pkg_route).or(ws_route);
    warp::serve(routes).run(([0, 0, 0, 0], 8088));
  }
}

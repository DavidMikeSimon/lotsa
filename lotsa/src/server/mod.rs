use std::{io::Write, sync::atomic::{AtomicUsize, Ordering}, sync::Arc, sync::Mutex};

use bincode::serialize;
use flate2::{write::ZlibEncoder, Compression};
use futures::{future::Future, stream::Stream, sync::mpsc};
use warp::Filter;
use tokio;

use crate::{block::EMPTY, chunk::Chunk, debug::Debugger, life::LIFE};

struct WebsocketConn {
  id: usize,
  tx: mpsc::UnboundedSender<warp::ws::Message>,
}

struct WebsocketServer {
  next_conn_id: AtomicUsize,
  conns: Mutex<Vec<Arc<WebsocketConn>>>
}

impl WebsocketServer {
  fn new() -> WebsocketServer {
    WebsocketServer {
      next_conn_id: AtomicUsize::new(1),
      conns: Mutex::new(Vec::new())
    }
  }

  fn accept_connection(
    self: Arc<WebsocketServer>,
    websocket: warp::ws::WebSocket,
  ) -> impl Future<Item = (), Error = ()> + 'static {
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

    self.conns.lock().expect("conns list lock infallible").push(conn.clone());

    info!("there are now {} connections", self.conns.lock().unwrap().len());

    let self2 = self.clone();
    socket_rx
      .for_each(move |msg| {
        self.received_msg(&conn, &msg);
        Ok(())
      })
      .then(move |result| {
        self2.lost_connection(id);
        result
      })
      .map_err(|e| {
        error!("websocket error: {:?}", e);
      })
  }

  fn received_msg(&self, conn: &WebsocketConn, _msg: &warp::ws::Message) {
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
    let msg = warp::ws::Message::binary(bytes);

    // If we can't sent the message, e.g. if client disconnected, don't worry about it
    let _ = conn.tx.unbounded_send(msg);
  }

  fn lost_connection(self: Arc<WebsocketServer>, id: usize) {
    info!("lost connection #{}", id);
    let mut conns = self.conns.lock().expect("conns list lock infallible");
    match conns.iter().position(|conn| conn.id == id) {
      Some(idx) => { conns.swap_remove(idx); () },
      None => (),
    };
    info!("there are now {} connections", conns.len());
  }
}

pub struct Server {
  ws_server: Arc<WebsocketServer>,
}

impl Server {
  pub fn new() -> Server {
    Server {
      ws_server: Arc::new(WebsocketServer::new()),
    }
  }

  pub fn start(&self) {
    if let Err(_) = std::env::var("RUST_LOG") {
      std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let root_route = warp::fs::dir("www");
    let pkg_route = warp::path("pkg").and(warp::fs::dir("pkg"));

    let ws_server = self.ws_server.clone();
    let ws_server_state_route = warp::any().map(move || ws_server.clone());
    let ws_route = warp::path("ws")
      .and(warp::ws2())
      .and(ws_server_state_route)
      .map(|ws: warp::ws::Ws2, ws_server: Arc<WebsocketServer>| {
        ws.on_upgrade(move |websocket| ws_server.accept_connection(websocket))
      });

    let routes = root_route.or(pkg_route).or(ws_route);
    warp::serve(routes).run(([0, 0, 0, 0], 8088));
  }
}

use std::rc::Rc;

use js_sys::{ArrayBuffer, JsString, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
  BinaryType, CanvasRenderingContext2d, HtmlCanvasElement, MessageEvent, Url, WebSocket,
};

use crate::{
  block::{EMPTY, UNKNOWN},
  chunk::{Chunk, CHUNK_WIDTH},
  life::LIFE,
};

#[wasm_bindgen]
pub struct LotsaClientWrapper {
  client: Rc<LotsaClient>,
  _rx_closure: Closure<dyn Fn(MessageEvent)>,
}

#[wasm_bindgen]
impl LotsaClientWrapper {
  #[allow(clippy::new_without_default)]
  #[wasm_bindgen(constructor)]
  pub fn new() -> LotsaClientWrapper {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("console_log init must succeed");

    let client = Rc::new(LotsaClient::new());

    let rx_closure = {
      let client = client.clone();
      Closure::wrap(
        Box::new(move |msg: MessageEvent| client.handle_message(msg)) as Box<dyn Fn(MessageEvent)>,
      )
    };
    client.ws.set_onmessage(Some(
      rx_closure
        .as_ref()
        .dyn_ref()
        .expect("closure can always be ref"),
    ));

    LotsaClientWrapper {
      client,
      _rx_closure: rx_closure,
    }
  }

  pub fn send_message(&self, data: &str) {
    self.client.ws.send_with_str(data).unwrap();
  }
}

struct LotsaClient {
  ws: WebSocket,
  canvas_ctx: CanvasRenderingContext2d,
  canvas_width: u32,
  canvas_height: u32,
}

const GRID: f64 = 2.0;

impl LotsaClient {
  pub fn new() -> LotsaClient {
    let window = web_sys::window().expect("dom window must exist");
    let document = window.document().expect("dom document must exist");

    let url = Url::new(&document.url().expect("dom document must have url")).expect("url is valid");
    let ws_url = format!("ws://{}/ws/", url.host());
    info!("websocket url {}", ws_url);

    let ws = WebSocket::new(&ws_url).expect("network is reliable"); // TODO
    ws.set_binary_type(BinaryType::Arraybuffer);

    let canvas_height = window
      .inner_height()
      .expect("dom window must have height")
      .as_f64()
      .expect("dom window height must be numeric") as u32;
    let canvas_width = canvas_height;

    let canvas = document
      .get_element_by_id("main")
      .expect("main element must exist")
      .dyn_into::<HtmlCanvasElement>()
      .expect("main element must be a canvas");

    canvas.set_width(canvas_width);
    canvas.set_height(canvas_height);

    let canvas_ctx = canvas
      .get_context("2d")
      .expect("get_context on canvas must succeed")
      .expect("get_context on canvas must return an object")
      .dyn_into::<CanvasRenderingContext2d>()
      .expect("get_context on canvas must return a canvas context");

    LotsaClient {
      ws,
      canvas_ctx,
      canvas_width,
      canvas_height,
    }
  }

  fn handle_message(&self, msg: MessageEvent) {
    let js_ab: ArrayBuffer = msg.data().into();
    let js_a: Uint8Array = Uint8Array::new(&js_ab);
    let mut buf: Vec<u8> = vec![0; js_a.length() as usize];
    js_a.copy_to(&mut buf[..]);

    let decoder = flate2::read::ZlibDecoder::new(&buf[..]);
    let chunk: Chunk =
      bincode::deserialize_from(decoder).expect("message is valid gzipped bincode Chunk"); // TODO

    self.draw(&chunk);
  }

  fn draw(&self, chunk: &Chunk) {
    self.canvas_ctx.begin_path();

    self
      .canvas_ctx
      .set_fill_style(JsString::from("#eee").as_ref());
    self.canvas_ctx.fill_rect(
      0.0,
      0.0,
      self.canvas_width as f64,
      self.canvas_height as f64,
    );

    let cell_width: f64 = ((self.canvas_width as f64 - GRID) / (CHUNK_WIDTH as f64) - GRID) as f64;
    let cell_height: f64 =
      ((self.canvas_height as f64 - GRID) / (CHUNK_WIDTH as f64) - GRID) as f64;

    for block in chunk.blocks_iter() {
      let pos = block.pos();
      if pos.z() > 0 {
        continue;
      }

      let color_str = match block.block_type() {
        EMPTY => "#fff",
        UNKNOWN => "#f00",
        LIFE => "#00f",
        _ => "#0f0",
      };
      self
        .canvas_ctx
        .set_fill_style(JsString::from(color_str).as_ref());

      let x: f64 = GRID + (pos.x() as f64) * (cell_width + GRID);
      let y: f64 = GRID + (pos.y() as f64) * (cell_height + GRID);
      self.canvas_ctx.fill_rect(x, y, cell_width, cell_height);
    }

    self.canvas_ctx.stroke();
  }
}

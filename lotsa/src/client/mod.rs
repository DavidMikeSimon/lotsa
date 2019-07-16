use std::rc::Rc;

use js_sys::{ArrayBuffer, JsString, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{BinaryType, CanvasRenderingContext2d, HtmlCanvasElement, MessageEvent, WebSocket};

use crate::{
  block::{EMPTY, UNKNOWN},
  chunk::Chunk,
  debug::Debugger,
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

  pub fn send_message(&self, data: &str) { self.client.ws.send_with_str(data).unwrap(); }
}

struct LotsaClient {
  ws: WebSocket,
  canvas_ctx: CanvasRenderingContext2d,
  canvas_width: u32,
  canvas_height: u32,
}

impl LotsaClient {
  pub fn new() -> LotsaClient {
    let ws = WebSocket::new("ws://localhost:8088/ws/").expect("network is reliable"); // TODO
    ws.set_binary_type(BinaryType::Arraybuffer);

    let window = web_sys::window().expect("dom window must exist");
    let canvas_height = window
      .inner_height()
      .expect("dom window must have height")
      .as_f64()
      .expect("dom window height must be numeric") as u32;
    let canvas_width = canvas_height;

    let canvas = window
      .document()
      .expect("dom document must exist")
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

    self.canvas_ctx.begin_path();
    self
      .canvas_ctx
      .set_fill_style(JsString::from("#0000ff").as_ref());
    self.canvas_ctx.fill_rect(50.0, 50.0, 100.0, 100.0);
    self.canvas_ctx.stroke();

    let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', LIFE => 'L'));
    info!("Chunk contains:\n{}", &debugger.dump(&chunk));
  }
}

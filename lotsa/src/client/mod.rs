use std::rc::Rc;

use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{BinaryType, MessageEvent, WebSocket};

use crate::{
  block::{EMPTY, UNKNOWN},
  chunk::Chunk,
  debug::Debugger,
  life::LIFE,
};

#[wasm_bindgen]
pub struct LotsaClient {
  core: Rc<LotsaClientCore>,
}

#[wasm_bindgen]
impl LotsaClient {
  #[allow(clippy::new_without_default)]
  #[wasm_bindgen(constructor)]
  pub fn new() -> LotsaClient {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("set up console_log");

    let core = Rc::new(LotsaClientCore {
      ws: WebSocket::new("ws://localhost:8088/ws/").expect("establish connection"),
    });
    let core2 = core.clone();

    let c = Closure::wrap(
      Box::new(move |msg: MessageEvent| core2.handle_message(msg)) as Box<dyn Fn(MessageEvent)>
    );
    core.ws.set_binary_type(BinaryType::Arraybuffer);
    core.ws.set_onmessage(Some(c.as_ref().unchecked_ref()));
    c.forget(); // FIXME: Maybe keep it in LotsaClient instead? Need
                // Rc<RefCell<LotsaClientCore>>?

    LotsaClient { core }
  }

  pub fn send_message(&self, data: &str) { self.core.ws.send_with_str(data).unwrap(); }
}

struct LotsaClientCore {
  ws: WebSocket,
}

impl LotsaClientCore {
  fn handle_message(&self, msg: MessageEvent) {
    let js_ab: ArrayBuffer = msg.data().into();
    let js_a: Uint8Array = Uint8Array::new(&js_ab);
    let mut buf: Vec<u8> = vec![0; js_a.length() as usize];
    js_a.copy_to(&mut buf[..]);

    let decoder = flate2::read::ZlibDecoder::new(&buf[..]);
    let chunk: Chunk = bincode::deserialize_from(decoder).expect("valid bincode");

    let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', LIFE => 'L'));
    info!("Chunk contains:\n{}", &debugger.dump(&chunk));
  }
}

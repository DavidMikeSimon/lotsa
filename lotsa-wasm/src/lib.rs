#[macro_use]
extern crate maplit;

use std::rc::Rc;

use bincode::deserialize;
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, BinaryType, MessageEvent, WebSocket};

use lotsa::{
  block::{EMPTY, UNKNOWN},
  chunk::Chunk,
  debug::Debugger,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct LotsaClient {
  core: Rc<LotsaClientCore>,
}

#[wasm_bindgen]
impl LotsaClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> LotsaClient {
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

    let chunk: Chunk = deserialize(&buf[..]).unwrap();

    let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.'));

    console::log_2(&"Chunk contents".into(), &debugger.dump(&chunk).into());
  }
}

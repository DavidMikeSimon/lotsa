use wasm_bindgen::prelude::*;
use web_sys::{WebSocket};
use js_sys::{Function};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub struct LotsaClient {
  ws: WebSocket,
}

#[wasm_bindgen]
impl LotsaClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> LotsaClient {
    let client = LotsaClient {
      ws: WebSocket::new("ws://localhost:8088/ws/").expect("establish connection"),
    };


    // let c = Closure::wrap(Box::new(move || alert("Hi") ));
    // client.ws.set_onmessage(Some(c));
    // c.forget();

    client
  }

  pub fn handle_message(&self, data: &str) {
    self.ws.send_with_str("HIYA").unwrap();
    alert(data);
  }
}

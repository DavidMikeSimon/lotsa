use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{MessageEvent, WebSocket};

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
    if let Some(s) = msg.data().as_string() {
      alert(&s);
    } else {
      alert("NOT A STRING");
    }
  }
}

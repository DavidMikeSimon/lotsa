#[macro_use]
extern crate maplit;

use wasm_bindgen::prelude::*;

use lotsa::{
  block::{BlockType, EMPTY, UNKNOWN},
  chunk::Chunk,
  debug::Debugger,
};

pub const STUFF: BlockType = BlockType(3);

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
pub fn greet() {
  let mut chunk = Chunk::new();
  let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', STUFF => 'S'));

  debugger.load(
    &mut chunk,
    ".....
     .....
     .SSS.
     .....
     .....",
  );

  alert(&debugger.dump(&chunk));
}

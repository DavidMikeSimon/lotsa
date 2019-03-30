#[macro_use]
extern crate maplit;

use actix_web::{server, App, HttpRequest};

use lotsa::{
  block::{BlockType, EMPTY, UNKNOWN},
  chunk::Chunk,
  debug::Debugger,
};

pub const STUFF: BlockType = BlockType(3);

fn index(_req: &HttpRequest) -> String {
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

  debugger.dump(&chunk)
}

pub fn main() {
  server::new(|| App::new().resource("/", |r| r.f(index)))
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
}

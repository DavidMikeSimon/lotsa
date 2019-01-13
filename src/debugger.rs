use std::collections::HashMap;

use UNKNOWN;
use EMPTY;
use BlockType;
use Chunk;
use Point;

pub struct Debugger {
  block_type_chars: HashMap<BlockType, char>,
}

impl Debugger {
  pub fn new(block_type_chars: HashMap<BlockType, char>) -> Debugger {
    Debugger { block_type_chars: block_type_chars }
  }

  pub fn bounds(&self, c: &Chunk) -> Point {
    let mut r = Point::new(0, 0, 0);

    for block in c.blocks_iter().filter(|b| b.block_type() != EMPTY && b.block_type() != UNKNOWN) {
      let p = block.pos();
      if p.x() > r.x() { r = Point::new(p.x(), r.y(), r.z()) }
      if p.y() > r.y() { r = Point::new(r.x(), p.y(), r.z()) }
      if p.z() > r.z() { r = Point::new(r.x(), r.y(), p.z()) }
    }

    r
  }

  pub fn dump(&self, c: &Chunk) -> String {
    let bounds = self.bounds(c);
    if bounds.z() != 0 {
      panic!("Cannot dump chunk unless all interesting blocks are on z layer 0")
    }

    let mut s = String::new();
    for y in 0..=bounds.y() {
      for x in 0..=bounds.x() {
        let block = c.get_block(Point::new(x, y, 0));
        let chr = self.block_type_chars.get(&block.block_type()).unwrap();
        s.push(*chr);
      }
      s.push('\n');
    }
    s
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use UNKNOWN;
  use EMPTY;
  use Chunk;
  use Point;

  const COBBLE: BlockType = BlockType(37);

  #[test]
  fn test_get_bounds() {
    let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', COBBLE => 'C'));
    let mut c = Chunk::new();
    c.set_block_type(Point::new(1, 1, 1), COBBLE);
    c.set_block_type(Point::new(1, 4, 2), COBBLE);
    c.set_block_type(Point::new(1, 2, 3), COBBLE);
    assert_eq!(debugger.bounds(&c), Point::new(1, 4, 3));
  }

  #[test]
  fn test_dump_2d() {
    let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', COBBLE => 'C'));
    let mut c = Chunk::new();
    for x in 0..5 {
      for y in 0..4 {
        println!("EMPTY {} {}", x, y);
        c.set_block_type(Point::new(x, y, 0), EMPTY);
      }
    }
    c.set_block_type(Point::new(1, 1, 0), COBBLE);
    c.set_block_type(Point::new(2, 3, 0), COBBLE);
    c.set_block_type(Point::new(4, 2, 0), COBBLE);

    assert_eq!(
      debugger.dump(&c),
      ".....\n\
       .C...\n\
       ....C\n\
       ..C..\n"
    )
  }
}

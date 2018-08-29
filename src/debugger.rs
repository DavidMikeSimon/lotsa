use std::collections::HashMap;

use UNKNOWN;
use EMPTY;
use BlockType;
use BlocksMatchingIterator;
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

    for block in c.blocks_matching(|b| b.block_type() != EMPTY && b.block_type() != UNKNOWN) {
      let p = block.pos();
      if p.x() > r.x() { r = Point::new(p.x(), r.y(), r.z()) }
      if p.y() > r.y() { r = Point::new(r.x(), p.y(), r.z()) }
      if p.z() > r.z() { r = Point::new(r.x(), r.y(), p.z()) }
    }

    r
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use UNKNOWN;
  use EMPTY;
  use Chunk;
  use Point;
  use life::LIFE;

  const COBBLE: BlockType = BlockType(37);

  #[test]
  fn test_get_bounds() {
    let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', COBBLE => 'C'));
    let mut c = Chunk::new();
    c.set_block_type(Point::new(1, 1, 1), LIFE);
    c.set_block_type(Point::new(1, 4, 2), LIFE);
    c.set_block_type(Point::new(1, 2, 3), LIFE);
    assert_eq!(debugger.bounds(&c), Point::new(1, 4, 3));
  }
}
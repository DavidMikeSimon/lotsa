use std::collections::HashMap;

use block::UNKNOWN;
use block::EMPTY;
use block::BlockType;
use chunk::Chunk;
use point::Point;

 #[macro_export]
 macro_rules! assert_trimmed_eq {
  ($a:expr, $b:expr) => {
    assert_eq!($a.trim().replace(" ", ""), $b.trim().replace(" ", ""))
  }
}

pub struct Debugger {
  block_type_chars: HashMap<BlockType, char>,
  char_block_types: HashMap<char, BlockType>,
}

impl Debugger {
  pub fn new(block_type_chars: HashMap<BlockType, char>) -> Debugger {
    let mut char_block_types: HashMap<char, BlockType> = HashMap::new();
    for (&bt, &c) in block_type_chars.iter() {
      if char_block_types.contains_key(&c) {
        panic!("Ambiguous mapping for Debugger character {}", c);
      }
      char_block_types.insert(c, bt);
    }

    Debugger {
      block_type_chars: block_type_chars,
      char_block_types: char_block_types,
    }
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

  pub fn load(&self, c: &mut Chunk, s: &str) {
    let mut x = 0;
    let mut y = 0;

    for chr in s.trim().chars() {
      match chr {
        ' ' => (), // Ignore spaces
        '\n' => {
          x = 0;
          y += 1;
        },
        _ => {
          let &bt = self.char_block_types.get(&chr).unwrap();
          c.set_block_type(Point::new(x, y, 0), bt);
          x += 1;
        },
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use block::UNKNOWN;
  use block::EMPTY;
  use chunk::Chunk;
  use point::Point;

  const COBBLE: BlockType = BlockType(37);

  fn build_debugger() -> Debugger {
    Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', COBBLE => 'C'))
  }

  #[test]
  fn test_get_bounds() {
    let debugger = build_debugger();
    let mut c = Chunk::new();
    c.set_block_type(Point::new(1, 1, 1), COBBLE);
    c.set_block_type(Point::new(1, 4, 2), COBBLE);
    c.set_block_type(Point::new(1, 2, 3), COBBLE);
    assert_eq!(debugger.bounds(&c), Point::new(1, 4, 3));
  }

  #[test]
  fn test_dump_2d() {
    let debugger = build_debugger();
    let mut c = Chunk::new();
    for x in 0..6 {
      for y in 0..6 {
        c.set_block_type(Point::new(x, y, 0), EMPTY);
      }
    }
    c.set_block_type(Point::new(1, 1, 0), COBBLE);
    c.set_block_type(Point::new(2, 3, 0), COBBLE);
    c.set_block_type(Point::new(4, 2, 0), COBBLE);

    assert_trimmed_eq!(debugger.dump(&c), "
      .....
      .C...
      ....C
      ..C..
    ")
  }

  #[test]
  fn test_load_2d() {
    let debugger = build_debugger();
    let mut c = Chunk::new();

    debugger.load(&mut c, "
      .....
      .CC.C
      ..CC.
      C....
    ");

    assert_eq!(c.get_block(Point::new(0, 0, 0)).block_type(), EMPTY);
    assert_eq!(c.get_block(Point::new(0, 0, 1)).block_type(), UNKNOWN);
    assert_eq!(c.get_block(Point::new(15, 0, 0)).block_type(), UNKNOWN);
    assert_eq!(c.get_block(Point::new(1, 1, 0)).block_type(), COBBLE);
    assert_eq!(c.get_block(Point::new(2, 1, 0)).block_type(), COBBLE);
    assert_eq!(c.get_block(Point::new(1, 2, 0)).block_type(), EMPTY);
    assert_eq!(c.get_block(Point::new(2, 2, 0)).block_type(), COBBLE);
    assert_eq!(c.get_block(Point::new(0, 3, 0)).block_type(), COBBLE);

    assert_trimmed_eq!(debugger.dump(&c), "
      .....
      .CC.C
      ..CC.
      C....
    ");
  }
}

#![feature(test)]

mod debugger;
mod life;

#[cfg(test)] #[macro_use] extern crate maplit;

extern crate test;

use std::ops::Index;
use std::ops::IndexMut;

pub const CHUNK_WIDTH: u8 = 32;
pub const CHUNK_WIDTH_E2: usize = (CHUNK_WIDTH as usize)*(CHUNK_WIDTH as usize);
pub const CHUNK_WIDTH_E3: usize = (CHUNK_WIDTH as usize)*CHUNK_WIDTH_E2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockType(pub u8);

impl std::convert::From<BlockType> for i32 {
  fn from(bt: BlockType) -> Self {
    bt.0 as i32
  }
}

pub const UNKNOWN: BlockType = BlockType(0);
pub const EMPTY: BlockType = BlockType(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
  n: u16,
}

impl Point {
  pub fn new(x: u8, y: u8, z: u8) -> Self {
    if x >= CHUNK_WIDTH { panic!("x is out of range") }
    if y >= CHUNK_WIDTH { panic!("y is out of range") }
    if z >= CHUNK_WIDTH { panic!("z is out of range") }
    Point { n: ((x as usize)*CHUNK_WIDTH_E2 + (y as usize)*(CHUNK_WIDTH as usize) + (z as usize)) as u16 }
  }

  pub fn x(&self) -> u8 {
    ((self.n/(CHUNK_WIDTH_E2 as u16)) % (CHUNK_WIDTH as u16)) as u8
  }

  pub fn y(&self) -> u8 {
    ((self.n/(CHUNK_WIDTH as u16)) % (CHUNK_WIDTH as u16)) as u8
  }

  pub fn z(&self) -> u8 {
    (self.n % (CHUNK_WIDTH as u16)) as u8
  }

  pub fn increment(&mut self) -> bool {
    if self.n == CHUNK_WIDTH_E3 as u16 - 1 {
      return false;
    } else {
      self.n += 1;
      return true;
    }
  }
}

#[derive(Clone, Copy)]
pub struct BlockView<'a> {
  chunk: &'a Chunk,
  block_type: BlockType,
  pos: Point,
}

impl<'a> BlockView<'a> {
  pub fn block_type(&self) -> BlockType { self.block_type }
  pub fn pos(&self) -> Point { self.pos }
}

type BlockTypesArray = [BlockType; CHUNK_WIDTH_E3 as usize];

impl Index<Point> for BlockTypesArray {
  type Output = BlockType;

  fn index(&self, pos: Point) -> &BlockType {
    self.get(pos.n as usize).unwrap()
  }
}

impl IndexMut<Point> for BlockTypesArray {
  fn index_mut(&mut self, pos: Point) -> &mut BlockType {
    self.get_mut(pos.n as usize).unwrap()
  }
}

#[derive(Clone, Copy)]
pub struct Chunk {
  block_types: BlockTypesArray,
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      block_types: [UNKNOWN; CHUNK_WIDTH_E3],
    }
  }

  pub fn get_block(&self, pos: Point) -> BlockView {
    BlockView {
      chunk: self,
      block_type: self.block_types[pos],
      pos: pos,
    }
  }

  pub fn set_block_type(&mut self, pos: Point, block_type: BlockType) {
    self.block_types[pos] = block_type;
  }

  pub fn blocks_matching(&self, condition: BlockCondition) -> BlocksMatchingIterator {
    BlocksMatchingIterator::new(self, condition)
  }
}

pub type BlockCondition = for<'a> fn(BlockView) -> bool;

pub struct BlocksMatchingIterator<'a> {
  chunk: &'a Chunk,
  pos: Point,
  done: bool,
  condition: BlockCondition,
}

impl<'a> BlocksMatchingIterator<'a> {
  fn new(chunk: &'a Chunk, condition: BlockCondition) -> BlocksMatchingIterator {
    BlocksMatchingIterator {
      chunk: chunk,
      pos: Point::new(0, 0, 0),
      done: false,
      condition: condition,
    }
  }
}

impl<'a> Iterator for BlocksMatchingIterator<'a> {
  type Item = BlockView<'a>;

  fn next(&mut self) -> Option<BlockView<'a>> {
    loop {
      if self.done { return None; }
      let block = self.chunk.get_block(self.pos);

      let incremented = self.pos.increment();
      if !incremented { self.done = true; }
      if (self.condition)(block) { return Some(block); }
    }
  }
}

impl<'a> std::iter::FusedIterator for BlocksMatchingIterator<'a> {}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  const COBBLE: BlockType = BlockType(37);

  #[test]
  fn test_point_splitting() {
    let p = Point::new(0, 0, 0);
    assert_eq!(p.x(), 0);
    assert_eq!(p.y(), 0);
    assert_eq!(p.z(), 0);

    let p = Point::new(1, 2, 3);
    assert_eq!(p.x(), 1);
    assert_eq!(p.y(), 2);
    assert_eq!(p.z(), 3);
  }

  #[test]
  fn test_get_block() {
    let c = Chunk::new();
    let block = c.get_block(Point::new(1, 2, 3));
    assert_eq!(block.block_type(), UNKNOWN);
    assert_eq!(block.pos(), Point::new(1, 2, 3));
  }

  #[test]
  fn test_set_block_type() {
    let mut c = Chunk::new();
    let p = Point::new(0, 0, 0);
    assert_eq!(c.get_block(p).block_type, UNKNOWN);
    c.set_block_type(p, COBBLE);
    assert_eq!(c.get_block(p).block_type, COBBLE);
  }

  #[test]
  fn test_get_matching_blocks() {
    let mut c = Chunk::new();
    c.set_block_type(Point::new(1, 1, 1), COBBLE);
    c.set_block_type(Point::new(2, 2, 2), COBBLE);
    c.set_block_type(Point::new(3, 3, 3), COBBLE);

    fn is_cobble(b: BlockView) -> bool { b.block_type == COBBLE };

    let mut iter = c.blocks_matching(is_cobble);
    assert_eq!(iter.next().unwrap().pos(), Point::new(1, 1, 1));
    assert_eq!(iter.next().unwrap().pos(), Point::new(2, 2, 2));
    assert_eq!(iter.next().unwrap().pos(), Point::new(3, 3, 3));
    assert!(iter.next().is_none());
  }

  #[bench]
  fn bench_get_matching_blocks(b: &mut Bencher) {
    let mut c = Chunk::new();
    c.set_block_type(Point::new(1, 1, 1), COBBLE);
    c.set_block_type(Point::new(2, 2, 2), COBBLE);
    c.set_block_type(Point::new(3, 3, 3), COBBLE);

    fn is_cobble(b: BlockView) -> bool { b.block_type == COBBLE };

    b.iter(|| {
      let mut iter = c.blocks_matching(is_cobble);
      iter.next();
      iter.next();
      iter.next();
      iter.next();
    });
  } 
}
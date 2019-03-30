use std::{
  iter::FusedIterator,
  ops::{Index, IndexMut},
};

use crate::{
  block::{BlockType, UNKNOWN},
  point::Point,
};

pub const CHUNK_WIDTH: u8 = 32;
pub const CHUNK_WIDTH_E2: usize = (CHUNK_WIDTH as usize) * (CHUNK_WIDTH as usize);
pub const CHUNK_WIDTH_E3: usize = (CHUNK_WIDTH as usize) * CHUNK_WIDTH_E2;

type BlockTypesArray = [BlockType; CHUNK_WIDTH_E3 as usize];

impl Index<Point> for BlockTypesArray {
  type Output = BlockType;

  fn index(&self, pos: Point) -> &BlockType { self.get(pos.raw_n() as usize).unwrap() }
}

impl IndexMut<Point> for BlockTypesArray {
  fn index_mut(&mut self, pos: Point) -> &mut BlockType {
    self.get_mut(pos.raw_n() as usize).unwrap()
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

  pub fn get_block(&self, pos: Point) -> BlockView<'_> {
    BlockView {
      chunk: self,
      block_type: self.block_types[pos],
      pos,
    }
  }

  pub fn set_block_type(&mut self, pos: Point, block_type: BlockType) {
    self.block_types[pos] = block_type;
  }

  pub fn blocks_iter(&self) -> ChunkBlocksIterator<'_> { ChunkBlocksIterator::new(self) }

  pub fn neighbor_types(&self, pos: Point) -> Vec<BlockType> {
    let mut r = Vec::new();

    for &y_offset in [-1, 0, 1].iter() {
      let tgt_y = pos.y() as isize + y_offset;
      if tgt_y >= 0 && tgt_y <= 15 {
        for &x_offset in [-1, 0, 1].iter() {
          if x_offset == 0 && y_offset == 0 {
            continue;
          }
          let tgt_x = pos.x() as isize + x_offset;
          if tgt_x >= 0 && tgt_x <= 15 {
            let neighbor_pos = Point::new(tgt_x as u8, tgt_y as u8, pos.z());
            r.push(self.get_block(neighbor_pos).block_type);
          }
        }
      }
    }

    r
  }
}

impl Default for Chunk {
  fn default() -> Self { Self::new() }
}

pub struct ChunkBlocksIterator<'a> {
  chunk: &'a Chunk,
  pos: Point,
  done: bool,
}

impl<'a> ChunkBlocksIterator<'a> {
  fn new(chunk: &'a Chunk) -> ChunkBlocksIterator<'_> {
    ChunkBlocksIterator {
      chunk,
      pos: Point::new(0, 0, 0),
      done: false,
    }
  }
}

impl<'a> Iterator for ChunkBlocksIterator<'a> {
  type Item = BlockView<'a>;

  fn next(&mut self) -> Option<BlockView<'a>> {
    if self.done {
      return None;
    }

    let block = self.chunk.get_block(self.pos);
    let incremented = self.pos.increment();
    if !incremented {
      self.done = true;
    }
    Some(block)
  }
}

impl<'a> FusedIterator for ChunkBlocksIterator<'a> {}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  const COBBLE: BlockType = BlockType(37);

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
  fn test_get_blocks() {
    let c = three_cobble_chunk();

    let mut iter = c.blocks_iter().filter(|b| b.block_type == COBBLE);
    assert_eq!(iter.next().unwrap().pos(), Point::new(1, 1, 0));
    assert_eq!(iter.next().unwrap().pos(), Point::new(2, 2, 0));
    assert_eq!(iter.next().unwrap().pos(), Point::new(3, 3, 0));
    assert!(iter.next().is_none());
  }

  #[test]
  fn test_neighbor_types() {
    let c = three_cobble_chunk();

    assert_eq!(
      c.neighbor_types(Point::new(2, 2, 0)),
      vec![COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE]
    );

    assert_eq!(
      c.neighbor_types(Point::new(9, 9, 0)),
      vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN]
    );

    // Top left corner
    assert_eq!(
      c.neighbor_types(Point::new(0, 0, 0)),
      vec![UNKNOWN, UNKNOWN, COBBLE]
    );

    // Top side
    assert_eq!(
      c.neighbor_types(Point::new(1, 0, 0)),
      vec![UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN]
    );

    // Left side
    assert_eq!(
      c.neighbor_types(Point::new(0, 1, 0)),
      vec![UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN]
    );
  }

  #[bench]
  fn bench_get_blocks(b: &mut Bencher) {
    let c = three_cobble_chunk();

    b.iter(|| {
      let mut iter = c.blocks_iter().filter(|b| b.block_type == COBBLE);
      iter.next();
      iter.next();
      iter.next();
      iter.next();
    });
  }

  fn three_cobble_chunk() -> Chunk {
    let mut c = Chunk::new();
    c.set_block_type(Point::new(1, 1, 0), COBBLE);
    c.set_block_type(Point::new(2, 2, 0), COBBLE);
    c.set_block_type(Point::new(3, 3, 0), COBBLE);
    c
  }
}

#[derive(Clone, Copy)]
pub struct BlockView<'a> {
  #[allow(dead_code)] // FIXME
  chunk: &'a Chunk,
  pub block_type: BlockType,
  pub pos: Point,
}

impl<'a> BlockView<'a> {
  pub fn block_type(&self) -> BlockType { self.block_type }

  pub fn pos(&self) -> Point { self.pos }
}

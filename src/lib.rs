pub const CHUNK_WIDTH: u8 = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
  x: u8,
  y: u8,
  z: u8,
}

impl Point {
  pub fn new(x: u8, y: u8, z: u8) -> Self {
    Point { x: x, y: y, z: z }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct BlockView<'a> {
  chunk: &'a Chunk,
  block_type: BlockType,
  pos: Point,
}

impl<'a> BlockView<'a> {
  pub fn block_type(&self) -> BlockType { self.block_type }
  pub fn pos(&self) -> Point { self.pos }
}

#[derive(Clone, Copy, Debug)]
pub struct Chunk {
  block_types: [[[BlockType; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      block_types: [[[UNKNOWN; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize]; CHUNK_WIDTH as usize],
    }
  }

  pub fn get_block(&self, pos: Point) -> BlockView {
    BlockView {
      chunk: self,
      block_type: self.block_types[pos.x as usize][pos.y as usize][pos.z as usize],
      pos: pos,
    }
  }

  pub fn set_block_type(&mut self, pos: Point, block_type: BlockType) {
    self.block_types[pos.x as usize][pos.y as usize][pos.z as usize] = block_type;
  }

  pub fn blocks_matching(&self, condition: BlockCondition) -> BlocksMatchingIterator {
    BlocksMatchingIterator {
      chunk: self,
      pos: Point { x: 0, y: 0, z: 0 },
      condition: condition,
    }
  }
}

pub type BlockCondition = for<'a> fn(BlockView) -> bool;

pub struct BlocksMatchingIterator<'a> {
  chunk: &'a Chunk,
  pos: Point,
  condition: BlockCondition,
}

impl<'a> Iterator for BlocksMatchingIterator<'a> {
  type Item = BlockView<'a>;

  fn next(&mut self) -> Option<BlockView<'a>> {
    loop {
      let block = self.chunk.get_block(self.pos);

      self.pos.x += 1;
      if self.pos.x == CHUNK_WIDTH {
        self.pos.x = 0;
        self.pos.y += 1;
        if self.pos.y == CHUNK_WIDTH {
          self.pos.y = 0;
          self.pos.z += 1;
          if self.pos.z == CHUNK_WIDTH {
            return None;
          }
        }
      }

      if (self.condition)(block) { return Some(block); }
    }
  }
}


#[cfg(test)]
mod tests {
  use Point;
  use BlockType;
  use BlockView;
  use Chunk;
  use UNKNOWN;

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
}
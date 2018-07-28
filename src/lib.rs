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

#[derive(Clone, Copy, Debug)]
pub struct BlockView<'a> {
  chunk: &'a Chunk,
  block_type: BlockType,
  x: u8,
  y: u8,
  z: u8,
}

impl<'a> BlockView<'a> {
  pub fn block_type(&self) -> BlockType { self.block_type }
  pub fn x(&self) -> u8 { self.x }
  pub fn y(&self) -> u8 { self.y }
  pub fn z(&self) -> u8 { self.z }
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

  pub fn get_block(&self, x: u8, y: u8, z: u8) -> BlockView {
    BlockView {
      chunk: self,
      block_type: self.block_types[x as usize][y as usize][z as usize],
      x: x,
      y: y,
      z: z,
    }
  }

  pub fn set_block_type(&mut self, x: u8, y: u8, z: u8, block_type: BlockType) {
    self.block_types[x as usize][y as usize][z as usize] = block_type;
  }

  pub fn blocks_matching(&self, condition: BlockCondition) -> BlocksMatchingIterator {
    BlocksMatchingIterator {
      chunk: self,
      x: 0,
      y: 0,
      z: 0,
      condition: condition,
    }
  }
}

pub type BlockCondition = for<'a> fn(BlockView) -> bool;

pub struct BlocksMatchingIterator<'a> {
  chunk: &'a Chunk,
  x: u8,
  y: u8,
  z: u8,
  condition: BlockCondition,
}

impl<'a> Iterator for BlocksMatchingIterator<'a> {
  type Item = BlockView<'a>;

  fn next(&mut self) -> Option<BlockView<'a>> {
    loop {
      let block = self.chunk.get_block(self.x, self.y, self.z);

      self.x += 1;
      if self.x == CHUNK_WIDTH {
        self.x = 0;
        self.y += 1;
        if self.y == CHUNK_WIDTH {
          self.y = 0;
          self.z += 1;
          if self.z == CHUNK_WIDTH {
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
  use BlockType;
  use BlockView;
  use Chunk;
  use UNKNOWN;

  const COBBLE: BlockType = BlockType(37);

  #[test]
  fn test_get_block() {
    let c = Chunk::new();
    let block = c.get_block(1, 2, 3);
    assert_eq!(block.block_type(), UNKNOWN);
    assert_eq!(block.x(), 1);
    assert_eq!(block.y(), 2);
    assert_eq!(block.z(), 3);
  }

  #[test]
  fn test_set_block_type() {
    let mut c = Chunk::new();
    assert_eq!(c.get_block(0, 0, 0).block_type, UNKNOWN);
    c.set_block_type(0, 0, 0, COBBLE);
    assert_eq!(c.get_block(0, 0, 0).block_type, COBBLE);
  }

  #[test]
  fn test_get_matching_blocks() {
    let mut c = Chunk::new();
    c.set_block_type(1, 1, 1, COBBLE);
    c.set_block_type(2, 2, 2, COBBLE);
    c.set_block_type(3, 3, 3, COBBLE);

    fn is_cobble(b: BlockView) -> bool { b.block_type == COBBLE };

    let mut iter = c.blocks_matching(is_cobble);
    assert_eq!(iter.next().unwrap().x, 1);
    assert_eq!(iter.next().unwrap().x, 2);
    assert_eq!(iter.next().unwrap().x, 3);
    assert!(iter.next().is_none());
  }
}
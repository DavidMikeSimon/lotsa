pub const CHUNK_WIDTH: usize = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BlockType(pub u8);

impl std::convert::From<BlockType> for i32 {
  fn from(bt: BlockType) -> Self {
    bt.0 as i32
  }
}

pub const UNKNOWN: BlockType = BlockType(0);
pub const EMPTY: BlockType = BlockType(1);

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

pub struct Chunk {
  block_types: [[[BlockType; CHUNK_WIDTH]; CHUNK_WIDTH]; CHUNK_WIDTH],
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      block_types: [[[UNKNOWN; CHUNK_WIDTH]; CHUNK_WIDTH]; CHUNK_WIDTH],
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
}

#[cfg(test)]
mod tests {
  use BlockType;
  use Chunk;
  use UNKNOWN;

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
    c.set_block_type(0, 0, 0, BlockType(37));
    assert_eq!(c.get_block(0, 0, 0).block_type, BlockType(37));
  }

}

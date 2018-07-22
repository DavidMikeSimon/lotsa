const CHUNK_WIDTH: usize = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct BlockType(pub u8);

struct Chunk {
  block_types: [[[BlockType; CHUNK_WIDTH]; CHUNK_WIDTH]; CHUNK_WIDTH],
}

impl Chunk {
  fn new() -> Chunk {
    Chunk {
      block_types: [[[BlockType(0); CHUNK_WIDTH]; CHUNK_WIDTH]; CHUNK_WIDTH]
    }
  }

  fn get_block_type(&self, x: u8, y: u8, z: u8) -> BlockType {
    self.block_types[x as usize][y as usize][z as usize]
  }

  fn set_block_type(&mut self, x: u8, y: u8, z: u8, block_type: BlockType) {
    self.block_types[x as usize][y as usize][z as usize] = block_type;
  }
}

#[cfg(test)]
mod tests {
  use BlockType;
  use Chunk;

  #[test]
  fn test_set_block_type() {
    let mut c = Chunk::new();
    assert_eq!(c.get_block_type(0, 0, 0), BlockType(0));
    c.set_block_type(0, 0, 0, BlockType(37));
    assert_eq!(c.get_block_type(0, 0, 0), BlockType(37));
  }
}
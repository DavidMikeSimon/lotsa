extern crate prost;
#[macro_use]
extern crate prost_derive;

pub mod proto {
  include!(concat!(env!("OUT_DIR"), "/lotsa.proto.rs"));
}

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

pub struct Chunk {
  block_types: [[[BlockType; CHUNK_WIDTH]; CHUNK_WIDTH]; CHUNK_WIDTH],
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      block_types: [[[UNKNOWN; CHUNK_WIDTH]; CHUNK_WIDTH]; CHUNK_WIDTH],
    }
  }

  pub fn get_block_type(&self, x: u8, y: u8, z: u8) -> BlockType {
    self.block_types[x as usize][y as usize][z as usize]
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
  use proto;

  #[test]
  fn test_set_block_type() {
    let mut c = Chunk::new();
    assert_eq!(c.get_block_type(0, 0, 0), UNKNOWN);
    c.set_block_type(0, 0, 0, BlockType(37));
    assert_eq!(c.get_block_type(0, 0, 0), BlockType(37));
  }

  #[test]
  fn test_conditional_on_block_type() {
    let condition = proto::ConditionalExpression {
      left: Some(Box::new(proto::ValueExpression {
        e: Some(proto::value_expression::E::FetchBlockType(
          proto::value_expression::FetchBlockType { }
        )),
      })),
      operator: proto::conditional_expression::Operator::Eq as i32,
      right: Some(Box::new(proto::ValueExpression {
        e: Some(proto::value_expression::E::Constant(
          proto::GenericValue {
            v: Some(proto::generic_value::V::BlockType(i32::from(BlockType(37))))
          }
        ))
      }))
    };
  }
}

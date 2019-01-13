use std::convert::From;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockType(pub u8);

impl From<BlockType> for i32 {
  fn from(bt: BlockType) -> Self {
    bt.0 as i32
  }
}

pub const UNKNOWN: BlockType = BlockType(0);
pub const EMPTY: BlockType = BlockType(1);

use std::convert::From;

use serde::{Deserialize, Serialize};

use crate::unique_descrip::UniqueDescrip;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockType(pub u16);

impl UniqueDescrip for BlockType {
  fn unique_descrip(&self) -> String { format!("BlockType{}", self.0) }
}

impl From<BlockType> for i32 {
  fn from(bt: BlockType) -> Self { i32::from(bt.0) }
}

pub const UNKNOWN: BlockType = BlockType(1);
pub const EMPTY: BlockType = BlockType(2);

use crate::{query::*, relative_pos::*};

#[derive(Clone, Debug, PartialEq)]
pub struct GetBlockType {}

impl GetBlockType {
  pub const fn new() -> GetBlockType { GetBlockType {} }
}

impl Default for GetBlockType {
  fn default() -> GetBlockType { GetBlockType::new() }
}

impl GenericQuery for GetBlockType {
  fn cacheability(&self) -> Cacheability {
    UntilChangeInSelf {
      fields: vec![CacheableBlockType],
    }
  }
}

impl Query<BlockType> for GetBlockType {
  fn eval(&self, n: &dyn Context, pos: RelativePos) -> BlockType { n.get_block(pos).block_type }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    block::UNKNOWN,
    query::tests::{TestContext, COBBLE},
  };

  #[test]
  fn test_get_block_type() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);
    let west = RelativePos::new(-1, 0, 0);

    let get_block_type = GetBlockType::new();

    assert_eq!(get_block_type.eval(&context, origin), COBBLE);
    assert_eq!(get_block_type.eval(&context, west), UNKNOWN);

    assert_eq!(
      get_block_type.cacheability(),
      UntilChangeInSelf {
        fields: vec![CacheableBlockType]
      }
    );
  }
}

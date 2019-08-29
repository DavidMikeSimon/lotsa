use crate::{query::*, relative_pos::*};

pub struct GetBlockType {}

impl GetBlockType {
  pub fn new() -> GetBlockType {
    GetBlockType {}
  }
}

impl Default for GetBlockType {
  fn default() -> GetBlockType {
    GetBlockType::new()
  }
}

impl<'a> Query<'a, BlockType> for GetBlockType {
  fn eval(&self, n: &'a Context, pos: RelativePos) -> BlockType {
    n.get_block(pos).block_type
  }

  fn cacheability(&self) -> Cacheability {
    UntilChangeInSelf {
      fields: vec![CacheableBlockType],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::block::UNKNOWN;
  use crate::query::tests::{TestContext, COBBLE};

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

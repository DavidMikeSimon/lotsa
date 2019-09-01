use crate::{query::*, relative_pos::*};
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Constant<T> {
  value: T,
}

impl<T> Constant<T> {
  pub fn new(value: T) -> Constant<T> {
    Constant { value }
  }
}

impl<T> GenericQuery for Constant<T>
where
  T: Debug,
{
  fn cacheability(&self) -> Cacheability {
    Forever
  }
}

impl<T> Query<T> for Constant<T>
where
  T: Copy + Debug + PartialEq,
{
  fn eval(&self, _n: &Context, _pos: RelativePos) -> T {
    self.value
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::query::tests::{TestContext, COBBLE};

  #[test]
  fn test_integer_constant() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);

    let one: Constant<u32> = Constant::new(1);
    assert_eq!(one.eval(&context, origin), 1);
    assert_eq!(one.cacheability(), Forever);
  }

  #[test]
  fn test_block_type_constant() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);

    let cobble: Constant<BlockType> = Constant::new(COBBLE);
    assert_eq!(cobble.eval(&context, origin), COBBLE);
    assert_eq!(cobble.cacheability(), Forever);
  }
}

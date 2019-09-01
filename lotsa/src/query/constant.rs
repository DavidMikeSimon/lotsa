use std::fmt::Debug;
use crate::{query::*, relative_pos::*};

#[derive(Clone, Debug, PartialEq)]
pub struct Constant<T>
where
  T: Copy + Debug + PartialEq,
{
  value: T,
}

impl<T> Constant<T>
where
  T: Copy + Debug + PartialEq,
{
  pub fn new(value: T) -> Constant<T> {
    Constant { value }
  }
}

impl<T> GenericQuery for Constant<T>
where
  T: Copy + Debug + PartialEq,
{
}

impl<T> Query<T> for Constant<T>
where
  T: Copy + Debug + PartialEq,
{
  fn eval<'a>(&self, _n: &Context, _pos: RelativePos) -> T where T: 'a {
    self.value
  }

  fn cacheability(&self) -> Cacheability {
    Forever
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

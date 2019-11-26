use crate::{query::*, relative_pos::*, unique_descrip::UniqueDescrip};

#[derive(Clone)]
pub struct Constant<T: UniqueDescrip> {
  value: T,
}

impl<T> Constant<T>
where
  T: Copy + UniqueDescrip,
{
  // TODO: Const?
  pub fn new(value: T) -> Constant<T> { Constant { value } }
}

impl<T> UniqueDescrip for Constant<T>
where
  T: Copy + UniqueDescrip,
{
  fn unique_descrip(&self) -> String { self.value.unique_descrip() }
}

impl<T> GenericQuery for Constant<T>
where
  T: Copy + UniqueDescrip,
{
  fn cacheability(&self) -> Cacheability { Forever }
}

impl<'a, T: 'a> Query<'a, T> for Constant<T>
where
  T: Copy + UniqueDescrip,
{
  fn eval(&self, _n: &dyn Context, _pos: RelativePos) -> T { self.value }
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

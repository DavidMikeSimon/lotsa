use super::*;

pub struct Constant<T>
where
  T: Copy,
{
  value: T,
}

impl<T> Constant<T>
where
  T: Copy,
{
  pub fn new(value: T) -> Constant<T> {
    Constant { value }
  }
}

impl<'a, T> Expr<'a, T> for Constant<T>
where
  T: Copy,
{
  fn eval(&self, _n: &'a Context, _pos: RelativePos) -> T {
    self.value
  }

  fn cacheability(&self) -> Cacheability {
    Forever
  }
}

#[cfg(test)]
mod tests {
  use super::super::tests::{TestContext, COBBLE};
  use super::*;

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

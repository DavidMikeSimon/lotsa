use super::*;

pub struct Equals<'a, T: PartialEq, L: Expr<'a, T>, R: Expr<'a, T>> {
  left: &'a L,
  right: &'a R,
  phantom: PhantomData<T>,
}

impl<'a, T, L, R> Equals<'a, T, L, R>
where
  T: PartialEq,
  L: Expr<'a, T>,
  R: Expr<'a, T>,
{
  pub fn new(left: &'a L, right: &'a R) -> Equals<'a, T, L, R> {
    Equals {
      left,
      right,
      phantom: PhantomData,
    }
  }
}

impl<'a, T, L, R> Expr<'a, bool> for Equals<'a, T, L, R>
where
  T: PartialEq,
  L: Expr<'a, T>,
  R: Expr<'a, T>,
{
  fn eval(&self, n: &'a Context, pos: RelativePos) -> bool {
    self.left.eval(n, pos) == self.right.eval(n, pos)
  }

  fn cacheability(&self) -> Cacheability {
    Cacheability::intersection(&self.left.cacheability(), &self.right.cacheability())
  }
}

#[cfg(test)]
mod tests {
  use super::super::tests::{TestContext, COBBLE};
  use super::*;
  use crate::block::UNKNOWN;

  #[test]
  fn test_equals_integers() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);

    let one: Constant<u32> = Constant::new(1);
    let another_one: Constant<u32> = Constant::new(1);
    let two: Constant<u32> = Constant::new(2);

    assert!(Equals::new(&one, &one).eval(&context, origin));
    assert!(Equals::new(&one, &another_one).eval(&context, origin));
    assert!(!Equals::new(&one, &two).eval(&context, origin));
  }

  #[test]
  fn test_equals_block_types() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);
    let west = RelativePos::new(-1, 0, 0);

    let cobble: Constant<BlockType> = Constant::new(COBBLE);
    let unknown: Constant<BlockType> = Constant::new(UNKNOWN);

    assert!(Equals::new(&cobble, &cobble).eval(&context, origin));
    assert!(Equals::new(&cobble, &cobble).eval(&context, west));
    assert!(!Equals::new(&unknown, &cobble).eval(&context, origin));
    assert!(!Equals::new(&unknown, &cobble).eval(&context, west));

    let get_block_type = GetBlockType::new();

    assert!(Equals::new(&get_block_type, &cobble).eval(&context, origin));
    assert!(Equals::new(&cobble, &get_block_type).eval(&context, origin));
    assert!(!Equals::new(&get_block_type, &unknown).eval(&context, origin));
    assert!(!Equals::new(&unknown, &get_block_type).eval(&context, origin));

    assert!(!Equals::new(&get_block_type, &cobble).eval(&context, west));
    assert!(!Equals::new(&cobble, &get_block_type).eval(&context, west));
    assert!(Equals::new(&get_block_type, &unknown).eval(&context, west));
    assert!(Equals::new(&unknown, &get_block_type).eval(&context, west));
  }
}

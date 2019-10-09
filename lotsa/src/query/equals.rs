use crate::{query::*, relative_pos::*};
use std::fmt;

pub struct Equals<T, L, R> {
  left: L,
  right: R,
  _phantom: PhantomData<T>,
}

impl<'a, T: 'a, L, R> Equals<T, L, R>
where
  T: PartialEq,
  L: Query<'a, T>,
  R: Query<'a, T>,
{
  // TODO: Const
  pub fn new(left: &L, right: &R) -> Equals<T, L, R> {
    Equals {
      left: left.clone(),
      right: right.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<'a, T: 'a, L, R> GenericQuery for Equals<T, L, R>
where
  L: Query<'a, T>,
  R: Query<'a, T>,
{
  fn cacheability(&self) -> Cacheability {
    Cacheability::merge(&self.left.cacheability(), &self.right.cacheability())
  }
}

impl<'a, T: 'a, L, R> Query<'a, bool> for Equals<T, L, R>
where
  T: PartialEq,
  L: Query<'a, T>,
  R: Query<'a, T>,
{
  fn eval(&self, n: &'a dyn Context, pos: RelativePos) -> bool {
    self.left.eval(n, pos) == self.right.eval(n, pos)
  }
}

impl<'a, T: 'a, L, R> Clone for Equals<T, L, R>
where
  L: Query<'a, T>,
  R: Query<'a, T>,
{
  fn clone(self: &Self) -> Self {
    Equals {
      left: self.left.clone(),
      right: self.right.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<'a, T: 'a, L, R> fmt::Debug for Equals<T, L, R>
where
  L: Query<'a, T>,
  R: Query<'a, T>,
{
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt
      .debug_struct("Equals")
      .field("left", &self.left)
      .field("right", &self.right)
      .finish()
  }
}

impl<'a, T: 'a, L, R> PartialEq for Equals<T, L, R>
where
  L: Query<'a, T>,
  R: Query<'a, T>,
{
  fn eq(&self, other: &Self) -> bool {
    // TODO: This should be commutative, but Rust won't let me compare L with R
    self.left == other.left && self.right == other.right
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    block::UNKNOWN,
    query::tests::{TestContext, COBBLE},
  };

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

    assert_eq!(Equals::new(&one, &one).cacheability(), Forever);
    assert_eq!(Equals::new(&one, &two).cacheability(), Forever);
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

    assert_eq!(Equals::new(&cobble, &cobble).cacheability(), Forever);
    assert_eq!(Equals::new(&unknown, &cobble).cacheability(), Forever);

    let get_block_type = GetBlockType::new();

    assert!(Equals::new(&get_block_type, &cobble).eval(&context, origin));
    assert!(Equals::new(&cobble, &get_block_type).eval(&context, origin));
    assert!(!Equals::new(&get_block_type, &unknown).eval(&context, origin));
    assert!(!Equals::new(&unknown, &get_block_type).eval(&context, origin));

    assert!(!Equals::new(&get_block_type, &cobble).eval(&context, west));
    assert!(!Equals::new(&cobble, &get_block_type).eval(&context, west));
    assert!(Equals::new(&get_block_type, &unknown).eval(&context, west));
    assert!(Equals::new(&unknown, &get_block_type).eval(&context, west));

    assert_eq!(
      Equals::new(&get_block_type, &cobble).cacheability(),
      UntilChangeInSelf {
        fields: vec![CacheableBlockType]
      }
    );

    assert_eq!(
      Equals::new(&cobble, &get_block_type).cacheability(),
      UntilChangeInSelf {
        fields: vec![CacheableBlockType]
      }
    );
  }
}

use std::marker::PhantomData;

use crate::block::BlockType;

#[derive(Clone, Copy)]
pub struct BlockInfo {
  pub block_type: BlockType,
}

impl BlockInfo {
  pub fn block_type(&self) -> BlockType {
    self.block_type
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelativePos {
  pub x: i8,
  pub y: i8,
  pub z: i8,
}

impl RelativePos {
  pub fn new(x: i8, y: i8, z: i8) -> RelativePos {
    RelativePos { x, y, z }
  }
}

pub trait Context {
  fn get_neighbor_block(&self, pos: RelativePos) -> BlockInfo;
}

pub trait Area<I>
where
  I: Iterator<Item = RelativePos>,
{
  fn offsets(&self) -> Box<Iterator<Item = RelativePos>>;
}

pub trait Expr<T> {
  fn eval(&self, n: &Context, pos: RelativePos) -> T;
}

pub struct GetBlockType {}

impl GetBlockType {
  pub fn new() -> GetBlockType {
    GetBlockType {}
  }
}

impl Expr<BlockType> for GetBlockType {
  fn eval(&self, n: &Context, pos: RelativePos) -> BlockType {
    n.get_neighbor_block(pos).block_type
  }
}

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

impl<T> Expr<T> for Constant<T>
where
  T: Copy,
{
  fn eval(&self, _n: &Context, _pos: RelativePos) -> T {
    self.value
  }
}

pub struct Equals<'a, T: PartialEq, L: Expr<T> + 'a, R: Expr<T> + 'a> {
  left: &'a L,
  right: &'a R,
  phantom: PhantomData<T>
}

impl<'a, T, L, R> Equals<'a, T, L, R>
where
  T: PartialEq,
  L: Expr<T> + 'a,
  R: Expr<T> + 'a,
{
  pub fn new(left: &'a L, right: &'a R) -> Equals<'a, T, L, R> {
    Equals { left, right, phantom: PhantomData }
  }
}

impl<'a, T, L, R> Expr<bool> for Equals<'a, T, L, R>
where
  T: PartialEq,
  L: Expr<T> + 'a,
  R: Expr<T> + 'a,
{
  fn eval(&self, n: &Context, pos: RelativePos) -> bool {
    self.left.eval(n, pos) == self.right.eval(n, pos)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::block::UNKNOWN;

  const COBBLE: BlockType = BlockType(37);

  #[test]
  fn test_equals() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);

    let one: Constant<u32> = Constant::new(1);
    let another_one: Constant<u32> = Constant::new(1);
    let two: Constant<u32> = Constant::new(2);

    assert_eq!(true, Equals::new(&one, &one).eval(&context, origin));
    assert_eq!(true, Equals::new(&one, &another_one).eval(&context, origin));
    assert_eq!(false, Equals::new(&one, &two).eval(&context, origin));
  }

  struct TestContext {
  }

  impl Context for TestContext {
    fn get_neighbor_block(&self, pos: RelativePos) -> BlockInfo {
      if pos.x == 0 {
        BlockInfo { block_type: COBBLE }
      } else {
        BlockInfo { block_type: UNKNOWN }
      }
    }
  }
}

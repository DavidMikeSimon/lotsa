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
  fn get_block(&self, pos: RelativePos) -> BlockInfo;
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
    n.get_block(pos).block_type
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

pub struct Equals<'a, T: PartialEq, L: Expr<T>, R: Expr<T>> {
  left: &'a L,
  right: &'a R,
  phantom: PhantomData<T>
}

impl<'a, T, L, R> Equals<'a, T, L, R>
where
  T: PartialEq,
  L: Expr<T>,
  R: Expr<T>,
{
  pub fn new(left: &'a L, right: &'a R) -> Equals<'a, T, L, R> {
    Equals { left, right, phantom: PhantomData }
  }
}

impl<'a, T, L, R> Expr<bool> for Equals<'a, T, L, R>
where
  T: PartialEq,
  L: Expr<T>,
  R: Expr<T>,
{
  fn eval(&self, n: &Context, pos: RelativePos) -> bool {
    self.left.eval(n, pos) == self.right.eval(n, pos)
  }
}

pub struct Chebyshev2DNeighbors {
  distance: i8
}

impl Chebyshev2DNeighbors {
  pub fn new(distance: u8) -> Chebyshev2DNeighbors {
    Chebyshev2DNeighbors { distance: distance as i8 }
  }
}

impl Expr<Box<Iterator<Item=BlockInfo>>> for Chebyshev2DNeighbors {
  fn eval(&self, n: &Context, pos: RelativePos) -> impl Iterator<Item = BlockInfo> {
    (-self.distance..=self.distance).flat_map(|x_offset| {
      (-self.distance..=self.distance).map(|y_offset| {
        n.get_block(
          RelativePos::new(x_offset+pos.x, y_offset+pos.y, pos.z)
        )
      })
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::block::UNKNOWN;

  const COBBLE: BlockType = BlockType(37);

  #[test]
  fn test_equals_integers() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);

    let one: Constant<u32> = Constant::new(1);
    let another_one: Constant<u32> = Constant::new(1);
    let two: Constant<u32> = Constant::new(2);

    assert_eq!(true, Equals::new(&one, &one).eval(&context, origin));
    assert_eq!(true, Equals::new(&one, &another_one).eval(&context, origin));
    assert_eq!(false, Equals::new(&one, &two).eval(&context, origin));
  }

  #[test]
  fn test_equals_block_types() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);
    let west = RelativePos::new(-1, 0, 0);

    let cobble: Constant<BlockType> = Constant::new(COBBLE);
    let unknown: Constant<BlockType> = Constant::new(UNKNOWN);

    assert_eq!(true, Equals::new(&cobble, &cobble).eval(&context, origin));
    assert_eq!(true, Equals::new(&cobble, &cobble).eval(&context, west));
    assert_eq!(false, Equals::new(&unknown, &cobble).eval(&context, origin));
    assert_eq!(false, Equals::new(&unknown, &cobble).eval(&context, west));

    let get_block_type = GetBlockType::new();

    assert_eq!(true, Equals::new(&get_block_type, &cobble).eval(&context, origin));
    assert_eq!(true, Equals::new(&cobble, &get_block_type).eval(&context, origin));
    assert_eq!(false, Equals::new(&get_block_type, &unknown).eval(&context, origin));
    assert_eq!(false, Equals::new(&unknown, &get_block_type).eval(&context, origin));

    assert_eq!(false, Equals::new(&get_block_type, &cobble).eval(&context, west));
    assert_eq!(false, Equals::new(&cobble, &get_block_type).eval(&context, west));
    assert_eq!(true, Equals::new(&get_block_type, &unknown).eval(&context, west));
    assert_eq!(true, Equals::new(&unknown, &get_block_type).eval(&context, west));
  }

  // #[test]
  // fn test_chebyshev_2d_neighbors() {
    // let context = TestContext {};
    // let origin = RelativePos::new(0, 0, 0);
    // let west = RelativePos::new(-1, 0, 0);

    // let cobble: Constant<BlockType> = Constant::new(COBBLE);
    // let unknown: Constant<BlockType> = Constant::new(UNKNOWN);

    // let neighbor_types = Chebyshev2DNeighbors::new(1, &GetBlockType::new());
    // assert_eq!(
      // vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN],
      // neighbor_types.eval(&context, origin).collect()
    // );
    // assert_eq!(
      // vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN],
      // neighbor_types.eval(&context, west).collect()
    // );
  // }

  struct TestContext {
  }

  impl Context for TestContext {
    fn get_block(&self, pos: RelativePos) -> BlockInfo {
      if pos.x == 0 {
        BlockInfo { block_type: COBBLE }
      } else {
        BlockInfo { block_type: UNKNOWN }
      }
    }
  }
}

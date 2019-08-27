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

pub trait Expr<'a, T> {
  fn eval(&self, n: &'a Context, pos: RelativePos) -> T;
}

pub struct GetBlockType {}

impl GetBlockType {
  pub fn new() -> GetBlockType {
    GetBlockType {}
  }
}

impl<'a> Expr<'a, BlockType> for GetBlockType {
  fn eval(&self, n: &'a Context, pos: RelativePos) -> BlockType {
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

impl<'a, T> Expr<'a, T> for Constant<T>
where
  T: Copy,
{
  fn eval(&self, _n: &'a Context, _pos: RelativePos) -> T {
    self.value
  }
}

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
}

pub struct Chebyshev2DNeighbors<'a, T, E>
where
  E: Expr<'a, T>,
{
  distance: i8,
  map_expr: &'a E,
  phantom: PhantomData<T>,
}

impl<'a, T, E> Chebyshev2DNeighbors<'a, T, E>
where
  E: Expr<'a, T>,
{
  pub fn new(distance: u8, map_expr: &'a E) -> Chebyshev2DNeighbors<'a, T, E> {
    Chebyshev2DNeighbors {
      distance: distance as i8,
      map_expr,
      phantom: PhantomData,
    }
  }
}

impl<'a, T, E> Expr<'a, Box<Iterator<Item = T> + 'a>> for Chebyshev2DNeighbors<'a, T, E>
where
  E: Expr<'a, T>,
{
  fn eval(&self, n: &'a Context, pos: RelativePos) -> Box<Iterator<Item = T> + 'a> {
    let distance = self.distance;
    let map_expr = self.map_expr;
    Box::new((-distance..=distance).flat_map(move |y_offset| {
      (-distance..=distance).map(move |x_offset| {
        map_expr.eval(
          n,
          RelativePos::new(x_offset + pos.x, y_offset + pos.y, pos.z),
        )
      })
    }))
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

  #[test]
  fn test_chebyshev_2d_neighbors() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);
    let west = RelativePos::new(-1, 0, 0);
    let get_block_type = GetBlockType::new();

    let get_neighbor_types = Chebyshev2DNeighbors::new(1, &get_block_type);
    assert_eq!(
      vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN],
      get_neighbor_types
        .eval(&context, origin)
        .collect::<Vec<BlockType>>()
    );
    assert_eq!(
      vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN],
      get_neighbor_types
        .eval(&context, west)
        .collect::<Vec<BlockType>>()
    );

    let cobble: Constant<BlockType> = Constant::new(COBBLE);
    let equals_cobble = Equals::new(&get_block_type, &cobble);
    let cobble_neighbors = Chebyshev2DNeighbors::new(1, &equals_cobble);
    assert_eq!(
      vec![false, false, false, false, true, false, false, false, false],
      cobble_neighbors
        .eval(&context, origin)
        .collect::<Vec<bool>>()
    );
  }

  struct TestContext {}

  impl Context for TestContext {
    fn get_block(&self, pos: RelativePos) -> BlockInfo {
      if pos.x == 0 && pos.y == 0 && pos.z == 0 {
        BlockInfo { block_type: COBBLE }
      } else {
        BlockInfo {
          block_type: UNKNOWN,
        }
      }
    }
  }
}

use std::cmp::max;
use std::marker::PhantomData;

use crate::block::BlockType;

#[derive(Clone)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cacheability {
  DontCache,
  Forever,
  UntilChangeInSelf {
    fields: Vec<CacheableField>,
  },
  UntilChangeInChebyshevNeighborhood {
    distance: u8,
    fields: Vec<CacheableField>,
  },
}

use Cacheability::*;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum CacheableField {
  CacheableBlockType,
}

use CacheableField::*;

const NO_FIELDS: &[CacheableField] = &[];
const ALL_FIELDS: &[CacheableField] = &[CacheableBlockType];

impl CacheableField {
  fn merge(a: &[CacheableField], b: &[CacheableField]) -> Vec<CacheableField> {
    let mut new_fields = Vec::new();
    new_fields.extend_from_slice(a);
    new_fields.extend_from_slice(b);
    new_fields.sort_unstable();
    new_fields.dedup();
    new_fields
  }
}

impl Cacheability {
  pub fn intersection(a: &Cacheability, b: &Cacheability) -> Cacheability {
    match (a, b) {
      (DontCache, _) => DontCache,
      (_, DontCache) => DontCache,
      (Forever, _) => b.clone(),
      (_, Forever) => a.clone(),
      (UntilChangeInSelf { fields: fields_a }, UntilChangeInSelf { fields: fields_b }) => {
        UntilChangeInSelf {
          fields: CacheableField::merge(fields_a, fields_b),
        }
      }
      (_, _) => UntilChangeInChebyshevNeighborhood {
        distance: max(a.distance(), b.distance()),
        fields: CacheableField::merge(a.fields(), b.fields()),
      },
    }
  }

  pub fn distance(&self) -> u8 {
    match self {
      UntilChangeInChebyshevNeighborhood { distance, .. } => *distance,
      _ => 0,
    }
  }

  pub fn fields(&self) -> &[CacheableField] {
    match self {
      DontCache => ALL_FIELDS,
      Forever => NO_FIELDS,
      UntilChangeInSelf { fields } => &fields,
      UntilChangeInChebyshevNeighborhood { fields, .. } => &fields,
    }
  }
}

pub trait Expr<'a, T> {
  fn eval(&self, n: &'a Context, pos: RelativePos) -> T;

  fn cacheability(&self) -> Cacheability {
    DontCache
  }
}

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

impl<'a> Expr<'a, BlockType> for GetBlockType {
  fn eval(&self, n: &'a Context, pos: RelativePos) -> BlockType {
    n.get_block(pos).block_type
  }

  fn cacheability(&self) -> Cacheability {
    UntilChangeInSelf {
      fields: vec![CacheableBlockType],
    }
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

  fn cacheability(&self) -> Cacheability {
    Forever
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

  fn cacheability(&self) -> Cacheability {
    Cacheability::intersection(&self.left.cacheability(), &self.right.cacheability())
  }
}

pub struct Chebyshev2DNeighbors<'a, T, E>
where
  E: Expr<'a, T>,
{
  distance: u8,
  map_expr: &'a E,
  phantom: PhantomData<T>,
}

impl<'a, T, E> Chebyshev2DNeighbors<'a, T, E>
where
  E: Expr<'a, T>,
{
  pub fn new(distance: u8, map_expr: &'a E) -> Chebyshev2DNeighbors<'a, T, E> {
    if distance > 127 {
      panic!("Distance must be <= 127")
    }
    Chebyshev2DNeighbors {
      distance,
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
    Box::new(
      (-(distance as i8)..=(distance as i8)).flat_map(move |y_offset| {
        (-(distance as i8)..=(distance as i8)).map(move |x_offset| {
          map_expr.eval(
            n,
            RelativePos::new(x_offset + pos.x, y_offset + pos.y, pos.z),
          )
        })
      }),
    )
  }

  fn cacheability(&self) -> Cacheability {
    let map_expr_cacheability = self.map_expr.cacheability();
    UntilChangeInChebyshevNeighborhood {
      distance: self.distance + map_expr_cacheability.distance(),
      fields: map_expr_cacheability.fields().to_vec(),
    }
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

  // TODO next: Write tests for cacheability

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

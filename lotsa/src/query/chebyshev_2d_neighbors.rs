use std::cmp::PartialEq;
use std::fmt;
use std::marker::PhantomData;

use crate::{query::*, relative_pos::*};

pub struct Chebyshev2DNeighbors<T, E>
where
  E: Query<T>,
{
  distance: u8,
  map_expr: E,
  _phantom: PhantomData<T>,
}

impl<T, E> Chebyshev2DNeighbors<T, E>
where
  E: Query<T>,
{
  pub fn new(distance: u8, map_expr: &E) -> Chebyshev2DNeighbors<T, E> {
    if distance > 127 {
      panic!("Distance must be <= 127")
    }
    Chebyshev2DNeighbors {
      distance,
      map_expr: map_expr.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<T, E> GenericQuery for Chebyshev2DNeighbors<T, E>
where
  E: Query<T>,
{
}

impl<T, E> Query<Vec<T>> for Chebyshev2DNeighbors<T, E>
where
  E: Query<T>,
{
  fn eval(&self, n: &Context, pos: RelativePos) -> Vec<T> {
    let d = self.distance;
    Box::new(
      (-(d as i8)..=(d as i8)).flat_map(move |y_offset| {
        (-(d as i8)..=(d as i8)).map(move |x_offset| {
          self.map_expr.eval(
            n,
            RelativePos::new(x_offset + pos.x, y_offset + pos.y, pos.z),
          )
        })
      }),
    ).collect()
  }

  fn cacheability(&self) -> Cacheability {
    match self.map_expr.cacheability() {
      DontCache => DontCache,
      Forever => Forever,
      map_expr_cacheability => UntilChangeInChebyshevNeighborhood {
        distance: self.distance + map_expr_cacheability.distance(),
        fields: map_expr_cacheability.fields().to_vec(),
      },
    }
  }
}

impl<T, E> Clone for Chebyshev2DNeighbors<T, E>
where
  E: Query<T>,
{
  fn clone(self: &Self) -> Self {
    Chebyshev2DNeighbors {
      distance: self.distance,
      map_expr: self.map_expr.clone(),
      _phantom: PhantomData
    }
  }
}

impl<T, E> fmt::Debug for Chebyshev2DNeighbors<T, E>
where
  E: Query<T>,
{
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.debug_struct("Chebyshev2DNeighbors")
      .field("distance", &self.distance)
      .field("map_expr", &self.map_expr)
      .finish()
  }
}

impl<T, E> PartialEq for Chebyshev2DNeighbors<T, E>
where
  E: Query<T>,
{
  fn eq(&self, other: &Self) -> bool {
    self.distance == other.distance && self.map_expr == other.map_expr
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::block::UNKNOWN;
  use crate::query::tests::{TestContext, COBBLE};

  #[test]
  fn test_neighbors_block_types() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);
    let west = RelativePos::new(-1, 0, 0);
    let get_block_type = GetBlockType::new();

    let get_neighbor_types = Chebyshev2DNeighbors::new(1, &get_block_type);
    assert_eq!(
      vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN],
      get_neighbor_types
        .eval(&context, origin)
    );
    assert_eq!(
      vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN],
      get_neighbor_types
        .eval(&context, west)
    );

    assert_eq!(
      get_neighbor_types.cacheability(),
      UntilChangeInChebyshevNeighborhood {
        distance: 1,
        fields: vec![CacheableBlockType]
      }
    )
  }

  #[test]
  fn test_neighbors_block_type_equality() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);
    let get_block_type = GetBlockType::new();

    let cobble: Constant<BlockType> = Constant::new(COBBLE);
    let equals_cobble = Equals::new(&get_block_type, &cobble);
    let get_neighbor_cobbleness = Chebyshev2DNeighbors::new(1, &equals_cobble);
    assert_eq!(
      vec![false, false, false, false, true, false, false, false, false],
      get_neighbor_cobbleness
        .eval(&context, origin)
    );

    assert_eq!(
      get_neighbor_cobbleness.cacheability(),
      UntilChangeInChebyshevNeighborhood {
        distance: 1,
        fields: vec![CacheableBlockType]
      }
    )
  }

  #[test]
  fn test_distant_neighbors_block_type_equality() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);
    let get_block_type = GetBlockType::new();

    let cobble: Constant<BlockType> = Constant::new(COBBLE);
    let equals_cobble = Equals::new(&get_block_type, &cobble);
    let get_distant_neighbor_cobbleness = Chebyshev2DNeighbors::new(2, &equals_cobble);

    let mut expected_bools: Vec<bool> = Vec::new();
    expected_bools.extend_from_slice(&[false; 12]);
    expected_bools.extend_from_slice(&[true; 1]);
    expected_bools.extend_from_slice(&[false; 12]);
    assert_eq!(
      expected_bools,
      get_distant_neighbor_cobbleness
        .eval(&context, origin)
    );

    assert_eq!(
      get_distant_neighbor_cobbleness.cacheability(),
      UntilChangeInChebyshevNeighborhood {
        distance: 2,
        fields: vec![CacheableBlockType]
      }
    )
  }

  #[test]
  fn test_nested() {
    let context = TestContext {};
    let origin = RelativePos::new(0, 0, 0);

    let get_block_type = GetBlockType::new();
    let get_neighbor_types = Chebyshev2DNeighbors::new(1, &get_block_type);
    // This is a terrible and silly approach, but fun to test anyways
    let get_neighbors_neighbor_types = Chebyshev2DNeighbors::new(1, &get_neighbor_types);

    assert_eq!(
      vec![
        vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE],
        vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN],
        vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN],
        vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN],
        vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN],
        vec![UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN],
        vec![UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN],
        vec![UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN],
        vec![COBBLE, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN],
      ],
      get_neighbors_neighbor_types
        .eval(&context, origin)
    );

    assert_eq!(
      get_neighbors_neighbor_types.cacheability(),
      UntilChangeInChebyshevNeighborhood {
        distance: 2,
        fields: vec![CacheableBlockType]
      }
    )
  }
}

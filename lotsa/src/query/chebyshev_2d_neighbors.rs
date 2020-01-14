use std::marker::PhantomData;

use crate::{query::*, relative_pos::*, unique_descrip::UniqueDescrip};

pub struct Chebyshev2DNeighbors<T, E> {
  distance: u8,
  map_expr: E,
  _phantom: PhantomData<T>,
}

impl<'a, T: 'a, E> Chebyshev2DNeighbors<T, E>
where
  E: Query<'a, T>,
{
  // TODO: Const
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

impl<T, E> UniqueDescrip for Chebyshev2DNeighbors<T, E>
where
  E: UniqueDescrip,
{
  fn unique_descrip(&self) -> String {
    format!(
      "Chebyshev2DNeighbors( dist:{}, {} )",
      self.distance,
      self.map_expr.unique_descrip()
    )
  }
}

impl<'a, T: 'a, E> GenericQuery for Chebyshev2DNeighbors<T, E>
where
  E: Query<'a, T>,
{
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

impl<'a, T: 'a, E: 'a> Query<'a, Box<dyn Iterator<Item = T> + 'a>> for Chebyshev2DNeighbors<T, E>
where
  E: Query<'a, T>,
{
  fn eval(&'a self, n: &'a dyn Context, pos: RelativePos) -> Box<dyn Iterator<Item = T> + 'a> {
    let d = self.distance as i8;
    // TODO: Z
    Box::new((-d..=d).flat_map(move |y_offset| {
      (-d..=d).map(move |x_offset| {
        self.map_expr.eval(
          n,
          RelativePos::new(x_offset + pos.x, y_offset + pos.y, pos.z),
        )
      })
    }))
  }
}

impl<'a, T: 'a, E> Clone for Chebyshev2DNeighbors<T, E>
where
  E: Query<'a, T>,
{
  fn clone(self: &Self) -> Self {
    Chebyshev2DNeighbors {
      distance: self.distance,
      map_expr: self.map_expr.clone(),
      _phantom: PhantomData,
    }
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
        .collect::<Vec<BlockType>>()
    );
    assert_eq!(
      vec![UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, COBBLE, UNKNOWN, UNKNOWN, UNKNOWN],
      get_neighbor_types
        .eval(&context, west)
        .collect::<Vec<BlockType>>()
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
        .collect::<Vec<bool>>()
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
        .collect::<Vec<bool>>()
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
        .map(Iterator::collect)
        .collect::<Vec<Vec<BlockType>>>()
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

use std::marker::PhantomData;

use super::*;

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
  use super::super::tests::{TestContext, COBBLE};
  use super::*;
  use crate::block::UNKNOWN;

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
}

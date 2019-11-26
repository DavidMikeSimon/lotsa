use std::{cmp::max, marker::PhantomData};

use crate::{block::BlockType, relative_pos::RelativePos, unique_descrip::UniqueDescrip};

mod chebyshev_2d_neighbors;
pub use chebyshev_2d_neighbors::*;

mod constant;
pub use constant::*;

mod equals;
pub use equals::*;

mod get_block_type;
pub use get_block_type::*;

pub trait Context {
  fn get_block(&self, pos: RelativePos) -> BlockInfo;
}

pub trait GenericQuery: UniqueDescrip {
  fn cacheability(&self) -> Cacheability;
}

pub trait Query<'a, T: 'a>: GenericQuery + Clone {
  fn eval(&'a self, n: &'a dyn Context, pos: RelativePos) -> T;
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
  pub block_type: BlockType,
}

impl BlockInfo {
  pub fn block_type(&self) -> BlockType { self.block_type }
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
  pub fn merge(a: &Cacheability, b: &Cacheability) -> Cacheability {
    match (a, b) {
      (DontCache, _) => DontCache,
      (_, DontCache) => DontCache,
      (Forever, _) => b.clone(),
      (_, Forever) => a.clone(),
      (UntilChangeInSelf { fields: fields_a }, UntilChangeInSelf { fields: fields_b }) => {
        UntilChangeInSelf {
          fields: CacheableField::merge(fields_a, fields_b),
        }
      },
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::block::UNKNOWN;

  pub const COBBLE: BlockType = BlockType(37);

  pub struct TestContext {}

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

  #[test]
  fn test_generic_query_equality() {
    // TODO
    // let one: &GenericQuery = &Constant::new(1);
    // let two: &GenericQuery = &Constant::new(2);
    // let get_block_type: &GenericQuery = &GetBlockType::new();

    // assert_eq!(one, two);
    // assert_eq!(get_block_type, get_block_type);

    // assert_ne!(one, two);
    // assert_ne!(one, get_block_type);
  }
}

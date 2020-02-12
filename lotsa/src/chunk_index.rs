use roaring::RoaringBitmap;

use crate::{chunk_pos::ChunkPos, relative_pos::RelativePos};

#[derive(Clone, Debug)]
pub struct ChunkIndex {
  index: RoaringBitmap,
}

impl ChunkIndex {
  pub fn new() -> ChunkIndex {
    ChunkIndex {
      index: RoaringBitmap::new(),
    }
  }

  pub fn mark(&mut self, pos: ChunkPos) { self.index.insert(pos.raw_n() as u32); }

  pub fn mark_chebyshev_neighborhood(&mut self, pos: ChunkPos, distance: u8) {
    let d = distance as i8;
    for y_offset in -d..=d {
      for x_offset in -d..=d {
        for z_offset in -d..=d {
          let relative_pos = RelativePos::new(x_offset, y_offset, z_offset);
          match pos.offset(relative_pos) {
            None => (), // TODO: Propagate to neighboring chunk?
            Some(offset_pos) => self.mark(offset_pos),
          }
        }
      }
    }
  }

  pub fn consider(&self, pos: ChunkPos) -> bool { self.index.contains(pos.raw_n() as u32) }

  pub fn clear(&mut self) { self.index.clear(); }

  pub fn iter<'a>(&'a self) -> impl Iterator<Item = ChunkPos> + 'a {
    self
      .index
      .iter()
      .map(|n| ChunkPos::new_from_raw_n(n as u16))
  }
}

impl Default for ChunkIndex {
  fn default() -> ChunkIndex { ChunkIndex::new() }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_consider() {
    let mut index = ChunkIndex::new();

    let pos_a = ChunkPos::new(0, 1, 2);
    let pos_b = ChunkPos::new(3, 4, 5);

    assert_eq!(index.consider(pos_a), false);
    assert_eq!(index.consider(pos_b), false);

    index.mark(pos_a);

    assert_eq!(index.consider(pos_a), true);
    assert_eq!(index.consider(pos_b), false);
  }

  #[test]
  fn test_iteration() {
    let mut index = ChunkIndex::new();

    let pos_a = ChunkPos::new(0, 1, 2);
    let pos_b = ChunkPos::new(3, 4, 5);
    let pos_c = ChunkPos::new(15, 15, 15);

    index.mark(pos_a);
    index.mark(pos_b);
    index.mark(pos_c);

    let considerables: Vec<ChunkPos> = index.iter().collect();

    assert!(considerables.contains(&pos_a));
    assert!(considerables.contains(&pos_b));
    assert!(considerables.contains(&pos_c));
    assert!(considerables.len() < 8); // Allowed to have false positives
  }

  #[test]
  fn test_clear() {
    let mut index = ChunkIndex::new();

    let pos = ChunkPos::new(0, 1, 2);

    index.mark(pos);
    assert_eq!(index.consider(pos), true);
    index.clear();
    assert_eq!(index.consider(pos), false);
    assert_eq!(index.iter().collect::<Vec<ChunkPos>>().len(), 0);
  }

  #[test]
  fn test_mark_chebyshev() {
    let mut index = ChunkIndex::new();

    index.mark_chebyshev_neighborhood(ChunkPos::new(5, 5, 5), 2);

    assert_eq!(index.consider(ChunkPos::new(5, 5, 5)), true);
    assert_eq!(index.consider(ChunkPos::new(4, 4, 5)), true);
    assert_eq!(index.consider(ChunkPos::new(3, 3, 5)), true);
    assert_eq!(index.consider(ChunkPos::new(6, 6, 5)), true);
    assert_eq!(index.consider(ChunkPos::new(7, 7, 5)), true);
    assert_eq!(index.consider(ChunkPos::new(3, 7, 5)), true);
    assert_eq!(index.consider(ChunkPos::new(7, 3, 5)), true);
    assert_eq!(index.consider(ChunkPos::new(7, 7, 7)), true);

    assert_eq!(index.consider(ChunkPos::new(2, 2, 5)), false);
    assert_eq!(index.consider(ChunkPos::new(8, 8, 5)), false);
    assert_eq!(index.consider(ChunkPos::new(5, 8, 5)), false);
    assert_eq!(index.consider(ChunkPos::new(7, 7, 8)), false);
  }
}

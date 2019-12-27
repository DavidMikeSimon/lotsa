use roaring::RoaringBitmap;

use crate::{
  chunk_pos::ChunkPos,
};

#[derive(Clone)]
pub struct ChunkIndex {
  index: RoaringBitmap
}

impl ChunkIndex {
  pub fn new() -> ChunkIndex {
    ChunkIndex {
      index: RoaringBitmap::new(),
    }
  }

  pub fn mark(&mut self, pos: ChunkPos) {
    self.index.insert(pos.raw_n() as u32);
  }

  pub fn consider(&self, pos: ChunkPos) -> bool {
    self.index.contains(pos.raw_n() as u32)
  }

  pub fn clear(&mut self) {
    self.index.clear();
  }

  pub fn iter<'a>(&'a self) -> impl Iterator<Item=ChunkPos> + 'a {
    self.index.iter().map(|n| ChunkPos::new_from_raw_n(n as u16))
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
  fn test_clear() {
    let mut index = ChunkIndex::new();

    let pos = ChunkPos::new(0, 1, 2);

    index.mark(pos);
    assert_eq!(index.consider(pos), true);
    index.clear();
    assert_eq!(index.consider(pos), false);
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
}

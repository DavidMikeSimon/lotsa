use bitmaps::{Bits, Bitmap};
use typenum::U1024;

use crate::{
  chunk::{CHUNK_WIDTH, CHUNK_WIDTH_E2},
  chunk_pos::ChunkPos,
};

type ViewBitmapSize = U1024;
type ViewBitmap = Bitmap<ViewBitmapSize>;

#[derive(Clone)]
pub struct ChunkIndex {
  xy_view: ViewBitmap,
  yz_view: ViewBitmap,
}

impl ChunkIndex {
  pub fn new() -> ChunkIndex {
    //TODO: Static assertion?
    debug_assert_eq!(
      std::mem::size_of::<<ViewBitmapSize as Bits>::Store>(),
      CHUNK_WIDTH_E2/8
    );

    ChunkIndex {
      xy_view: ViewBitmap::new(),
      yz_view: ViewBitmap::new(),
    }
  }

  pub fn mark(&mut self, pos: ChunkPos) {
    self.xy_view.set((pos.x()*CHUNK_WIDTH + pos.y()) as usize, true);
    self.yz_view.set((pos.y()*CHUNK_WIDTH + pos.z()) as usize, true);
  }

  pub fn consider(&self, pos: ChunkPos) -> bool {
    self.xy_view.get((pos.x()*CHUNK_WIDTH + pos.y()) as usize)
    && self.yz_view.get((pos.y()*CHUNK_WIDTH + pos.z()) as usize)
  }

  pub fn clear(&mut self) {
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
}

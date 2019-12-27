use crate::{
  chunk::{CHUNK_WIDTH, CHUNK_WIDTH_E2, CHUNK_WIDTH_E3},
  relative_pos::RelativePos,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkPos {
  n: u16,
}

impl ChunkPos {
  pub fn new(x: u8, y: u8, z: u8) -> Self {
    debug_assert!(x < CHUNK_WIDTH);
    debug_assert!(y < CHUNK_WIDTH);
    debug_assert!(z < CHUNK_WIDTH);
    ChunkPos {
      n: ((x as usize) * CHUNK_WIDTH_E2 + (y as usize) * (CHUNK_WIDTH as usize) + (z as usize))
        as u16,
    }
  }

  pub fn raw_n(self) -> u16 { self.n }

  #[allow(clippy::cast_lossless)]
  pub fn x(self) -> u8 { ((self.n / (CHUNK_WIDTH_E2 as u16)) % (CHUNK_WIDTH as u16)) as u8 }

  #[allow(clippy::cast_lossless)]
  pub fn y(self) -> u8 { ((self.n / (CHUNK_WIDTH as u16)) % (CHUNK_WIDTH as u16)) as u8 }

  #[allow(clippy::cast_lossless)]
  pub fn z(self) -> u8 { (self.n % (CHUNK_WIDTH as u16)) as u8 }

  pub fn increment(&mut self) -> bool {
    if self.n == CHUNK_WIDTH_E3 as u16 - 1 {
      false
    } else {
      self.n += 1;
      true
    }
  }

  pub fn offset(&self, r: RelativePos) -> Option<ChunkPos> {
    let self_x = self.x() as i8;
    if r.x < 0 && r.x.abs() > self_x {
      return None;
    }
    let x = (self_x + r.x) as u8;
    if x >= CHUNK_WIDTH {
      return None;
    }

    let self_y = self.y() as i8;
    if r.y < 0 && r.y.abs() > self_y {
      return None;
    }
    let y = (self_y + r.y) as u8;
    if y >= CHUNK_WIDTH {
      return None;
    }

    let self_z = self.z() as i8;
    if r.z < 0 && r.z.abs() > self_z {
      return None;
    }
    let z = (self_z + r.z) as u8;
    if z >= CHUNK_WIDTH {
      return None;
    }

    Some(ChunkPos::new(x, y, z))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_point_splitting() {
    let p = ChunkPos::new(0, 0, 0);
    assert_eq!(p.x(), 0);
    assert_eq!(p.y(), 0);
    assert_eq!(p.z(), 0);

    let p = ChunkPos::new(1, 2, 3);
    assert_eq!(p.x(), 1);
    assert_eq!(p.y(), 2);
    assert_eq!(p.z(), 3);

    let p = ChunkPos::new(8, 9, 7);
    assert_eq!(p.x(), 8);
    assert_eq!(p.y(), 9);
    assert_eq!(p.z(), 7);

    let p = ChunkPos::new(15, 15, 15);
    assert_eq!(p.x(), 15);
    assert_eq!(p.y(), 15);
    assert_eq!(p.z(), 15);
  }

  #[test]
  fn test_offset() {
    let p = ChunkPos::new(1, 2, 3);

    assert_eq!(
      Some(ChunkPos::new(9, 8, 7)),
      p.offset(RelativePos::new(8, 6, 4))
    );

    assert_eq!(
      Some(ChunkPos::new(0, 1, 1)),
      p.offset(RelativePos::new(-1, -1, -2))
    );

    assert_eq!(
      Some(ChunkPos::new(31, 2, 3)),
      p.offset(RelativePos::new(30, 0, 0))
    );

    assert_eq!(None, p.offset(RelativePos::new(-2, 0, 0)));

    assert_eq!(None, p.offset(RelativePos::new(31, 0, 0)));
  }
}

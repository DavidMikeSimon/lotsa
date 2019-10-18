use crate::chunk::CHUNK_WIDTH;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelativePos {
  pub x: i8,
  pub y: i8,
  pub z: i8,
}

impl RelativePos {
  pub fn new(x: i8, y: i8, z: i8) -> RelativePos {
    debug_assert!(x.abs() < CHUNK_WIDTH as i8);
    debug_assert!(y.abs() < CHUNK_WIDTH as i8);
    debug_assert!(z.abs() < CHUNK_WIDTH as i8);
    RelativePos { x, y, z }
  }

  pub fn here() -> RelativePos { RelativePos::new(0, 0, 0) }
}

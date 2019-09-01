use crate::chunk::CHUNK_WIDTH;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelativePos {
  pub x: i8,
  pub y: i8,
  pub z: i8,
}

impl RelativePos {
  pub fn new(x: i8, y: i8, z: i8) -> RelativePos {
    if x.abs() >= CHUNK_WIDTH as i8 {
      panic!("x is out of range")
    }
    if y.abs() >= CHUNK_WIDTH as i8 {
      panic!("y is out of range")
    }
    if z.abs() >= CHUNK_WIDTH as i8 {
      panic!("z is out of range")
    }
    RelativePos { x, y, z }
  }

  pub fn here() -> RelativePos { RelativePos::new(0, 0, 0) }
}

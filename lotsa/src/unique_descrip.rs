pub trait UniqueDescrip {
  // TODO: Const?
  fn unique_descrip(&self) -> String;
}

impl UniqueDescrip for u8 {
  fn unique_descrip(&self) -> String { format!("{}u8", self) }
}

impl UniqueDescrip for u16 {
  fn unique_descrip(&self) -> String { format!("{}u16", self) }
}

impl UniqueDescrip for u32 {
  fn unique_descrip(&self) -> String { format!("{}u32", self) }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_descrip_u8() {
    assert_eq!("42u8", 42u8.unique_descrip());
  }

  #[test]
  fn test_descrip_u16() {
    assert_eq!("42u16", 42u16.unique_descrip());
  }

  #[test]
  fn test_descrip_u32() {
    assert_eq!("42u32", 42u32.unique_descrip());
  }
}

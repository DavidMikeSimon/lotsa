use block::BlockType;

pub const LIFE: BlockType = BlockType(2);

#[cfg(test)]
mod tests {
  #[test]
  fn test_blinker() {
    assert_eq!(1, 1);
  }
}

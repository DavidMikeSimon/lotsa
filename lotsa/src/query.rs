use std::marker::PhantomData;

use crate::{block::BlockType};

#[derive(Clone, Copy)]
pub struct BlockView {
  pub block_type: BlockType
}

impl BlockView {
  pub fn block_type(&self) -> BlockType {
    self.block_type
  }
}

pub trait Blocks<I>
where
  I: Iterator<Item = BlockView>,
{
  fn iter(&self) -> I;
}

pub trait Expr<T> {
  fn eval(&self) -> T;
}

pub struct Count<I, B>
where
  I: Iterator<Item = BlockView>,
  B: Blocks<I>,
{
  blocks: B,
  iter_type: PhantomData<I>,
}

impl<I, B> Count<I, B>
where
  I: Iterator<Item = BlockView>,
  B: Blocks<I>,
{
  fn new(blocks: B) -> Count<I, B> {
    Count {
      blocks,
      iter_type: PhantomData,
    }
  }
}

impl<I, B> Expr<usize> for Count<I, B>
where
  I: Iterator<Item = BlockView>,
  B: Blocks<I>,
{
  fn eval(&self) -> usize {
    self.blocks.iter().count()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::iter;

  const COBBLE: BlockType = BlockType(37);

  #[test]
  fn test_count_one() {
    let blocks = TestBlocks::new(&|| {
      iter::once(BlockView { block_type: COBBLE })
    });
    let count = Count::new(blocks);
    assert_eq!(count.eval(), 1);
    assert_eq!(count.eval(), 1);
  }

  #[test]
  fn test_count_zero() {
    let blocks = TestBlocks::new(&|| iter::empty());
    let count = Count::new(blocks);
    assert_eq!(count.eval(), 0);
    assert_eq!(count.eval(), 0);
  }

  struct TestBlocks<'a, I: Iterator<Item = BlockView>> {
    iter_maker: &'a Fn() -> I,
  }

  impl<'a, I> TestBlocks<'a, I>
  where
    I: Iterator<Item = BlockView>,
  {
    fn new(iter_maker: &'a Fn() -> I) -> TestBlocks<I> {
      TestBlocks { iter_maker }
    }
  }

  impl<I> Blocks<I> for TestBlocks<'_, I>
  where
    I: Iterator<Item = BlockView>,
  {
    fn iter(&self) -> I {
      (self.iter_maker)()
    }
  }
}

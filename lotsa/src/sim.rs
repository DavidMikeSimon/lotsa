use std::collections::HashMap;

use crate::{block::BlockType, chunk::Chunk, point::Point};

type UpdaterFn = fn(BlockType, &[BlockType]) -> Option<BlockType>;

pub struct Simulator {
  chunk: Chunk,
  updaters: HashMap<BlockType, Vec<UpdaterFn>>,
}

#[derive(Clone, Copy, Debug)]
struct BlockTypeUpdate {
  pos: Point,
  block_type: BlockType,
}

impl Simulator {
  pub fn new(chunk: &mut Chunk) -> Simulator {
    Simulator {
      chunk: *chunk,
      updaters: HashMap::new(),
    }
  }

  pub fn get_chunk(&self) -> &Chunk { &self.chunk }

  pub fn add_updater(&mut self, target: BlockType, updater: UpdaterFn) {
    self
      .updaters
      .entry(target)
      .or_insert_with(Vec::new)
      .push(updater);
  }

  pub fn step(&mut self) {
    let mut updates: Vec<BlockTypeUpdate> = Vec::new();

    for block in self.chunk.blocks_iter() {
      if let Some(updaters) = self.updaters.get(&block.block_type) {
        for updater in updaters {
          if let Some(new_block_type) =
            updater(block.block_type, &self.chunk.neighbor_types(block.pos))
          {
            updates.push(BlockTypeUpdate {
              pos: block.pos,
              block_type: new_block_type,
            });
          }
        }
      }
    }

    for update in updates {
      self.chunk.set_block_type(update.pos, update.block_type);
    }
  }
}

use std::collections::HashMap;

use crate::{block::BlockType, chunk::Chunk, chunk_pos::ChunkPos};

type UpdaterFn = fn(BlockType, &[BlockType]) -> Option<BlockType>;

pub struct Simulator {
  updaters: HashMap<BlockType, Vec<UpdaterFn>>,
}

#[derive(Clone, Copy, Debug)]
struct BlockTypeUpdate {
  pos: ChunkPos,
  block_type: BlockType,
}

impl Simulator {
  pub fn new() -> Simulator {
    Simulator {
      updaters: HashMap::new(),
    }
  }

  pub fn add_updater(&mut self, target: BlockType, updater: UpdaterFn) {
    self
      .updaters
      .entry(target)
      .or_insert_with(Vec::new)
      .push(updater);
  }

  pub fn step(&self, chunk: &mut Chunk) {
    let mut updates: Vec<BlockTypeUpdate> = Vec::new();

    for (pos, block) in chunk.blocks_iter() {
      if let Some(updaters) = self.updaters.get(&block.block_type) {
        for updater in updaters {
          if let Some(new_block_type) = updater(block.block_type, &chunk.neighbor_types(pos))
          {
            updates.push(BlockTypeUpdate {
              pos,
              block_type: new_block_type,
            });
          }
        }
      }
    }

    for update in updates {
      chunk.set_block_type(update.pos, update.block_type);
    }
  }
}

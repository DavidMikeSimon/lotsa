use std::{collections::HashMap, marker::PhantomData};

use crate::{
  block::{BlockType, UNKNOWN},
  chunk::Chunk,
  chunk_pos::ChunkPos,
  query::{BlockInfo, Context, Query},
  relative_pos::RelativePos,
};

pub struct Simulator {
  updaters: HashMap<BlockType, Vec<Updater>>,
}

pub struct Updater {}

impl Updater {
  fn new(target: BlockType) -> Updater { Updater {} }

  fn run(&self, chunk: &Chunk, pos: &ChunkPos) -> Option<BlockType> { None }

  pub fn prepare_query<Q, T>(&self, query: &Q) -> LinkedQuery<Q, T>
  where
    Q: Query<T>,
  {
    LinkedQuery::new(query)
  }

  pub fn implement(&self, updater_fn: impl Fn(&UpdaterHandle) -> Option<BlockType>) {}
}

pub struct LinkedQuery<Q, T>
where
  Q: Query<T>,
{
  query: Q,
  _phantom: PhantomData<T>,
}

impl<Q, T> LinkedQuery<Q, T>
where
  Q: Query<T>,
{
  fn new(query: &Q) -> LinkedQuery<Q, T> {
    LinkedQuery {
      query: query.clone(),
      _phantom: PhantomData,
    }
  }
}

pub struct UpdaterHandle {
  context: UpdaterContext,
}

impl UpdaterHandle {
  pub fn query<Q, T>(&self, linked_query: &LinkedQuery<Q, T>) -> T
  where
    Q: Query<T>,
  {
    linked_query.query.eval(&self.context, RelativePos::here())
  }
}

struct UpdaterContext {}

impl Context for UpdaterContext {
  fn get_block(&self, pos: RelativePos) -> BlockInfo {
    // TODO
    BlockInfo {
      block_type: UNKNOWN,
    }
  }
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

  pub fn add_updater(&mut self, target: BlockType, setup_fn: fn(&mut Updater)) {
    let mut updater = Updater::new(target);
    setup_fn(&mut updater);
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
          if let Some(new_block_type) = updater.run(&chunk, &pos) {
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

use std::{collections::HashMap, marker::PhantomData};

use crate::{
  block::{BlockType, UNKNOWN},
  chunk::Chunk,
  chunk_pos::ChunkPos,
  query::{BlockInfo, Context, Query},
  relative_pos::RelativePos,
};

pub struct Simulator {
  updaters: HashMap<BlockType, Vec<Box<Updater>>>,
}

// TODO: Use a builder pattern so that Updater doesn't need to have an Option
pub struct Updater {
  updater_fn: Option<Box<dyn Fn(&UpdaterHandle) -> Option<BlockType>>>,
}

impl Updater {
  fn new() -> Updater { Updater { updater_fn: None } }

  fn run(&self, chunk: &Chunk, pos: ChunkPos) -> Option<BlockType> {
    let handle = UpdaterHandle {
      context: UpdaterContext {
        chunk,
        chunk_pos: pos,
      },
    };
    self.updater_fn.as_ref().unwrap()(&handle)
  }

  pub fn prepare_query<'a, Q, T>(&self, query: &Q) -> PreparedQuery<'a, Q, T>
  where
    Q: Query<'a, T>,
  {
    PreparedQuery::new(query)
  }

  pub fn implement(&mut self, updater_fn: impl Fn(&UpdaterHandle) -> Option<BlockType> + 'static) {
    self.updater_fn = Some(Box::new(updater_fn))
  }
}

pub struct PreparedQuery<'a, Q, T: 'a>
where
  Q: Query<'a, T>,
{
  query: Q,
  _phantom: PhantomData<&'a T>,
}

impl<'a, Q, T> PreparedQuery<'a, Q, T>
where
  Q: Query<'a, T>,
{
  fn new(query: &Q) -> PreparedQuery<'a, Q, T> {
    PreparedQuery {
      query: query.clone(),
      _phantom: PhantomData,
    }
  }
}

pub struct UpdaterHandle<'a> {
  context: UpdaterContext<'a>,
}

impl<'a> UpdaterHandle<'a> {
  pub fn query<Q, T: 'a>(&'a self, linked_query: &'a PreparedQuery<'a, Q, T>) -> T
  where
    Q: Query<'a, T>,
  {
    linked_query.query.eval(&self.context, RelativePos::here())
  }
}

struct UpdaterContext<'a> {
  chunk: &'a Chunk,
  chunk_pos: ChunkPos,
}

impl<'a> Context for UpdaterContext<'a> {
  fn get_block(&self, rel_pos: RelativePos) -> BlockInfo {
    match self.chunk_pos.offset(rel_pos) {
      Some(pos) => self.chunk.get_block(pos),
      None => BlockInfo {
        block_type: UNKNOWN,
      },
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
    let mut updater = Box::new(Updater::new());
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
          if let Some(new_block_type) = updater.run(&chunk, pos) {
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

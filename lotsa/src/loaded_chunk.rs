use std::{collections::HashMap};

use crate::{
  block::BlockType,
  chunk::Chunk,
  chunk_index::ChunkIndex,
  chunk_pos::ChunkPos,
  query::{BlockInfo, Cacheability, CacheableField},
};

#[derive(Clone)]
pub struct LoadedChunk {
  chunk: Chunk,
  cache_busters: HashMap<Cacheability, ChunkIndex>,
}

impl LoadedChunk {
  pub fn new(chunk: Chunk) -> LoadedChunk {
    LoadedChunk {
      chunk,
      cache_busters: HashMap::new(),
    }
  }

  pub fn get(&self) -> &Chunk { &self.chunk }

  pub fn reset_cache_busters<'a, T: Iterator<Item = &'a Cacheability>>(
    &mut self,
    cacheabilities: T,
  ) {
    self.cache_busters = HashMap::new();
    for cacheability in cacheabilities {
      self
        .cache_busters
        .insert(cacheability.clone(), ChunkIndex::new());
    }
  }

  pub fn set_block_type(&mut self, pos: ChunkPos, block_type: BlockType) {
    self.chunk.set_block_type(pos, block_type);

    for (cacheability, chunk_index) in self.cache_busters.iter_mut() {
      match cacheability {
        // TODO: Shouldn't even bother to keep the two below in cache_busters
        Cacheability::DontCache => (),
        Cacheability::Forever => (),
        Cacheability::UntilChangeInSelf { fields } => {
          if fields.contains(&CacheableField::CacheableBlockType) {
            chunk_index.mark(pos);
          }
        },
        Cacheability::UntilChangeInChebyshevNeighborhood { fields, distance } => {
          if fields.contains(&CacheableField::CacheableBlockType) {
            chunk_index.mark_chebyshev_neighborhood(pos, *distance);
          }
        },
      }
    }
  }

  pub fn considerable_blocks_iter<'a>(
    &'a self,
    cacheability: &Cacheability,
  ) -> Box<dyn Iterator<Item = (ChunkPos, BlockInfo)> + 'a> {
    match cacheability {
      Cacheability::DontCache =>
        Box::new(self.chunk.blocks_iter()),
      Cacheability::Forever =>
        Box::new(self.chunk.blocks_iter()), // FIXME: Only first run
      _ => {
        match self.cache_busters.get(cacheability) {
          None => Box::new(self.chunk.blocks_iter()),
          Some(chunk_index) => {
            Box::new(
              chunk_index.iter().map(move |chunk_pos| (chunk_pos, self.chunk.get_block(chunk_pos)) )
            )
          }
        }
      }
    }
  }
}

use crate::{
  block::{BlockType, EMPTY},
  query::{Chebyshev2DNeighbors, GetBlockType},
  sim::{Simulator, UpdaterHandle},
};

pub const LIFE: BlockType = BlockType(3);

fn live_blocks_here(neighbors: &mut dyn Iterator<Item = BlockType>) -> usize {
  neighbors.filter(|&b| b == LIFE).count()
}

pub fn init(sim: &mut Simulator) {
  sim.add_updater(LIFE, |updater| {
    let neighbor_block_types =
      updater.prepare_query(&Chebyshev2DNeighbors::new(1, &GetBlockType::new()));
    updater.implement(move |handle: &UpdaterHandle| {
      let nearby = live_blocks_here(Box::leak(handle.query(&neighbor_block_types))) - 1;
      if nearby >= 2 && nearby <= 4 {
        None
      } else {
        Some(EMPTY)
      }
    });
  });

  sim.add_updater(EMPTY, |updater| {
    let neighbor_block_types =
      updater.prepare_query(&Chebyshev2DNeighbors::new(1, &GetBlockType::new()));
    updater.implement(move |handle: &UpdaterHandle| {
      let nearby = live_blocks_here(Box::leak(handle.query(&neighbor_block_types)));
      if nearby == 3 {
        Some(LIFE)
      } else {
        None
      }
    });
  });
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{block::UNKNOWN, chunk::Chunk, debug::Debugger, loaded_chunk::LoadedChunk};
  use test::Bencher;

  #[test]
  fn test_blinker() {
    let mut chunk = Chunk::new();
    let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', LIFE => 'L'));

    debugger.load(
      &mut chunk,
      ".....
       .....
       .LLL.
       .....
       .....",
    );

    let mut loaded_chunk = LoadedChunk::new(chunk);

    let mut sim = Simulator::new();
    init(&mut sim);

    sim.step(&mut loaded_chunk);
    debugger.assert_match(
      loaded_chunk.get(),
      ".....
       ..L..
       ..L..
       ..L..
       .....",
    );

    sim.step(&mut loaded_chunk);
    debugger.assert_match(
      loaded_chunk.get(),
      ".....
       .....
       .LLL.
       .....
       .....",
    );
  }

  #[bench]
  fn bench_blinker(b: &mut Bencher) {
    let mut base_chunk = Chunk::new();
    let debugger = Debugger::new(hashmap!(UNKNOWN => 'X', EMPTY => '.', LIFE => 'L'));

    debugger.load(
      &mut base_chunk,
      ".....
       .....
       .LLL.
       .....
       .....",
    );

    let mut sim = Simulator::new();
    init(&mut sim);

    b.iter(|| {
      let mut loaded_chunk = LoadedChunk::new(base_chunk.clone());

      for _ in 1..20 {
        sim.step(&mut loaded_chunk);
      }

      debugger.dump(loaded_chunk.get())
    });
  }
}

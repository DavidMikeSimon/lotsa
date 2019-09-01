use crate::{
  block::{BlockType, EMPTY},
  query::{Chebyshev2DNeighbors, GetBlockType},
  sim::Simulator,
};

pub const LIFE: BlockType = BlockType(3);

fn live_neighbors(neighbors: Vec<BlockType>) -> usize {
  neighbors.iter().filter(|&&b| b == LIFE).count()
}

pub fn init(sim: &mut Simulator) {
  sim.add_updater(LIFE, |updater| {
    let neighbor_block_types =
      updater.prepare_query(&Chebyshev2DNeighbors::new(1, &GetBlockType::new()));
    updater.implement(|handle| {
      let nearby = live_neighbors(handle.query(&neighbor_block_types));
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
    updater.implement(move |handle| {
      let nearby = live_neighbors(handle.query(&neighbor_block_types));
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
  use crate::{block::UNKNOWN, chunk::Chunk, debug::Debugger};
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

    let mut sim = Simulator::new();
    init(&mut sim);

    sim.step(&mut chunk);
    print!("{:?}", debugger.dump(&chunk));
    debugger.assert_match(
      &chunk,
      ".....
       ..L..
       ..L..
       ..L..
       .....",
    );

    sim.step(&mut chunk);
    debugger.assert_match(
      &chunk,
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

    b.iter(|| {
      let mut chunk = base_chunk;
      let mut sim = Simulator::new();
      init(&mut sim);

      for _x in 1..10 {
        sim.step(&mut chunk);
      }
    });
  }
}

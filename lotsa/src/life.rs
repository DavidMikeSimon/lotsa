use crate::{
  block::{BlockType, EMPTY},
  sim::Simulator,
};

pub const LIFE: BlockType = BlockType(3);

fn live_neighbors(neighbors: &[BlockType]) -> usize {
  neighbors.iter().filter(|&&b| b == LIFE).count()
}

fn update_life_block_type(_this: BlockType, neighbors: &[BlockType]) -> Option<BlockType> {
  let nearby = live_neighbors(neighbors);
  if nearby >= 2 && nearby <= 4 {
    None
  } else {
    Some(EMPTY)
  }
}

fn update_empty_block_type(_this: BlockType, neighbors: &[BlockType]) -> Option<BlockType> {
  if live_neighbors(neighbors) == 3 {
    Some(LIFE)
  } else {
    None
  }
}

pub fn init(sim: &mut Simulator) {
  sim.add_updater(LIFE, update_life_block_type);
  sim.add_updater(EMPTY, update_empty_block_type);
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

    let mut sim = Simulator::new(&mut chunk);
    init(&mut sim);

    print!("{:?}", debugger.dump(sim.get_chunk()));
    sim.step();
    print!("{:?}", debugger.dump(sim.get_chunk()));
    debugger.assert_match(
      sim.get_chunk(),
      ".....
       ..L..
       ..L..
       ..L..
       .....",
    );

    sim.step();
    debugger.assert_match(
      sim.get_chunk(),
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
      let mut sim = Simulator::new(&mut chunk);
      init(&mut sim);

      for _x in 1..10 {
        sim.step();
      }
    });
  }
}

use crate::block::BlockType;

pub const LIFE: BlockType = BlockType(2);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::block::{EMPTY, UNKNOWN};

  use crate::{chunk::Chunk, debug::Debugger, sim::Simulator};

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

    sim.step();
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
}

use chunk::Chunk;

pub struct Simulator {
  chunk: Chunk,
}

impl Simulator {
  pub fn new(chunk: &mut Chunk) -> Simulator {
    Simulator {
      chunk: *chunk,
    }
  }

  pub fn get_chunk(&self) -> &Chunk {
    &self.chunk
  }

  pub fn step(&mut self) {
  }
}

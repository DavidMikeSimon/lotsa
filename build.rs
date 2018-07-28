extern crate prost_build;

fn main() {
  // TODO: Remove these when prost_build does this on its own...
  println!("cargo:rerun-if-changed=proto");
  println!("cargo:rerun-if-changed=proto/lotsa.proto");

  prost_build::compile_protos(&["proto/lotsa.proto"], &["proto/"]).unwrap();
}
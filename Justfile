run: build
  cargo run --package lotsa-server

build:
  cargo build --package lotsa-server
  wasm-pack build lotsa-wasm
  npm --prefix www run build

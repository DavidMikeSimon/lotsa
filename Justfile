run: build
  # FIXME: This seems to be rebuilding the server... also why do we need to change directory?
  cd lotsa-example; cargo run --features server

build:
  cargo build --package lotsa-example --features server
  wasm-pack build --target web lotsa-example -- --features client

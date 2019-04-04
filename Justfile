# Using --manifest-path instead of --package due to known cargo issue with --feature:
# https://github.com/rust-lang/cargo/issues/5364

run: build
  cargo run --manifest-path lotsa-example/Cargo.toml --features server

build:
  cargo build --manifest-path lotsa-example/Cargo.toml --features server
  wasm-pack build --no-typescript --target web lotsa-example -- --features client

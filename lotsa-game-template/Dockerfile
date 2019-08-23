FROM rust:1.36

RUN cargo install just
RUN cargo install wasm-pack
RUN cargo install wasm-bindgen-cli
RUN rustup component add --target wasm32-unknown-unknown rust-std

EXPOSE 8000

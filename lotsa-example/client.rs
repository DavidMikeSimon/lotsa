mod game;

// FIXME: Why doesn't this seem to work with cfg(client)?
#[cfg(target_arch = "wasm32")]
pub use lotsa_client::LotsaClient;

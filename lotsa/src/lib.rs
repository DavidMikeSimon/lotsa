//#![feature(test)]
#![warn(unused)]
#![warn(future_incompatible)]
#![warn(clippy::all)]

#[cfg(feature = "client")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[allow(clippy::useless_attribute, unused)]
#[macro_use]
extern crate maplit;

#[allow(clippy::useless_attribute, unused)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_big_array;

#[cfg(test)]
extern crate test;

pub mod block;
pub mod chunk;
pub mod debug;
pub mod life;
pub mod point;
pub mod sim;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
pub mod server;

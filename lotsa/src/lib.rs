#![feature(test)]
#![warn(unused)]
#![warn(future_incompatible)]
#![warn(clippy::all)]

#[cfg(test)]
#[macro_use]
extern crate maplit;

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

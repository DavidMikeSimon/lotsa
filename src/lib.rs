#![feature(test)]

#[cfg(test)] #[macro_use] extern crate maplit;
#[cfg(test)] extern crate test;

pub mod block;
pub mod chunk;
pub mod debug;
pub mod point;
pub mod life;
pub mod sim;

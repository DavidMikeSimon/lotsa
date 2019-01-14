#![feature(test)]

#[cfg(test)] #[macro_use] extern crate maplit;

extern crate test;

#[macro_use]
mod debug;

mod block;
mod chunk;
mod point;
mod life;
mod sim;

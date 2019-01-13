#![feature(test)]

#[cfg(test)] #[macro_use] extern crate maplit;

extern crate itertools;
extern crate test;

mod block;
mod chunk;
mod debugger;
mod point;
mod life;

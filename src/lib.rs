#![feature(const_fn)]

pub mod game;
pub mod movements;
pub mod pieces;
pub mod rules;

extern crate rand;
#[cfg(test)]
#[macro_use]
extern crate double;

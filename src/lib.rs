#![feature(const_fn)]

pub mod game;
pub mod movements;
pub mod pieces;
pub mod rules;

#[cfg(test)]
#[macro_use]
extern crate double;
extern crate rand;

#![feature(nll)]
#![feature(const_fn)]
#![feature(test)]

pub mod game;
pub mod movements;
pub mod pieces;
pub mod rules;

extern crate rand;

#[cfg(test)]
extern crate test;
#[cfg(test)]
#[macro_use]
extern crate double;

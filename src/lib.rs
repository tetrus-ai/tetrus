// this is super weird that I have to do this
// surely the compiler is aware that these modules exist on the STD namespace?
mod square;
pub mod tetromino;
mod in_play;
pub mod tetromino_generator;
pub mod well;
pub mod game;

extern crate rand;
use rand::StdRng;

mod position;
mod up_next;
mod tetromino_stream;
mod placed_piece;

pub mod shape;

#[derive(Copy, Clone)]
pub struct TetrominoStream {
    rng: StdRng,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacedPiece {
    pub shape: Shape,
    pub position: Position,
}

#[derive(Clone, Copy)]
pub struct TetrominoBuffer {
    pub first: Shape,
    pub second: Shape,
    generator: TetrominoStream,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {}

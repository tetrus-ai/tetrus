use rand::StdRng;

mod position;
mod shape;
mod square;
mod up_next;
mod tetromino_generator;
mod placed_piece;

#[derive(Copy, Clone)]
pub struct TetrominoGenerator {
    rng: StdRng,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacedPiece {
    pub shape: Shape,
    pub position: Position,
}

#[derive(Clone, Copy)]
pub struct UpNext {
    pub first: Shape,
    pub second: Shape,
    generator: TetrominoGenerator,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {}

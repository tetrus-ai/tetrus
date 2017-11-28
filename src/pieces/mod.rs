use rand::StdRng;

mod position;
mod tetromino_buffer;
mod random_tetromino_stream;
mod placed_piece;

pub mod shape;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlacedPiece {
    pub shape: Shape,
    pub position: Position,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

#[derive(Copy, Clone, Debug)]
pub struct RandomTetrominoStream {
    rng: StdRng,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Shape {
    name: ShapeName
}

#[derive(Clone, Copy, Debug)]
pub struct RandomTetrominoBuffer {
    pub first: Shape,
    pub second: Shape,
    stream: RandomTetrominoStream
}

pub trait TetrominoStream: Copy + Clone {
    fn next(&self) -> (Self, Option<Shape>);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ShapeName {
    I,
    J,
    L,
    O,
    S,
    Z,
    T
}

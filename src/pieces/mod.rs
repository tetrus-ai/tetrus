use rand::StdRng;

mod position;
mod tetromino_buffer;
mod random_tetromino_stream;
mod placed_piece;

pub mod shape;

pub const ORIGIN: Position = Position {
    x: ORIGIN_X,
    y: ORIGIN_Y,
};

pub const ORIGIN_X: i8 = 0;
pub const ORIGIN_Y: i8 = 0;

pub const BOUNDARY_LEFT: i8 = -4;
pub const BOUNDARY_RIGHT: i8 = 4;
pub const BOUNDARY_BOTTOM: i8 = 20;

pub const MOVE_SPEED: i8 = 1;

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
pub struct RandomTetrominoServer {
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

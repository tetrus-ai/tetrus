use ::square::Square;
use ::position::Position;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tetromino {
}

impl Tetromino{
    fn new() -> Self{
        Tetromino{
        }
    }

    pub fn i() -> Self{
        Tetromino::new()
    }
    pub fn j() -> Self{
        Tetromino::new()
    }
    pub fn l() -> Self{
        Tetromino::new()
    }
    pub fn s() -> Self{
        Tetromino::new()
    }
    pub fn t() -> Self{
        Tetromino::new()
    }
    pub fn o() -> Self{
        Tetromino::new()
    }
    pub fn z() -> Self{
        Tetromino::new()
    }
}
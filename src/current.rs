use super::tetromino::Tetromino;
use super::position::Position;

pub struct Current{
    pub tetromino: Tetromino,
    pub position: Position
}

impl Current{
    pub fn new(tetromino: Tetromino) -> Self{
        Current{
            tetromino,
            position: Position::new(5, 2)
        }
    }

    pub fn dropByOne(&self) -> Current{
        let position = self.position.add_to_y(1);
        Current{
            tetromino: self.tetromino,
            position
        }
    }
}
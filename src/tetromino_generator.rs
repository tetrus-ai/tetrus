use super::tetromino::Tetromino;

#[derive(Default)]
pub struct TetrominoGenerator {
}

pub trait GenerateTetromino{
    fn next(&self) -> Tetromino;
}

impl GenerateTetromino for TetrominoGenerator {
    fn next(&self) -> Tetromino {
        unimplemented!()
    }
}
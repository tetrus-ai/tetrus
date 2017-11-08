use super::tetromino::Tetromino;

#[derive(Default)]
pub struct TetrominoGenerator {
}

pub trait GenerateTetromino{
    fn next(&self) -> Tetromino;
}

impl GenerateTetromino for TetrominoGenerator {
    fn next(&self) -> Tetromino {
        Tetromino::default()
    }
}

#[cfg(test)]
mod next_should {
    use super::TetrominoGenerator;
    use super::GenerateTetromino;
    use super::Tetromino;

    #[test]
    fn return_a_tetromino(){
        let generator = TetrominoGenerator::default();
        let tetromino = generator.next();

        assert_eq!(tetromino, Tetromino::default())
    }
}
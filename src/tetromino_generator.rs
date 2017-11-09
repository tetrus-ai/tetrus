use super::tetromino::Tetromino;
use super::rand::{StdRng, SeedableRng};

pub struct TetrominoGenerator {
    rng: StdRng
}

impl TetrominoGenerator{
    fn new(rng: StdRng) -> Self{
        TetrominoGenerator{
            rng
        }
    }
}

pub trait GenerateTetromino{
    fn next(&self) -> Tetromino;
}

impl GenerateTetromino for TetrominoGenerator {
    fn next(&self) -> Tetromino {
        Tetromino::default()
    }
}

impl Default for TetrominoGenerator {
    fn default() -> Self {
        TetrominoGenerator{
            rng: StdRng::new().unwrap()
        }
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
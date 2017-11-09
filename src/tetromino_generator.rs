use super::tetromino::Tetromino;
use super::rand::StdRng;

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
mod should {
    use super::TetrominoGenerator;
    use super::GenerateTetromino;
    use super::Tetromino;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn return_a_tetromino(){
        let rng = StdRng::from_seed(&[1usize]);
        let generator = TetrominoGenerator::new(rng);
        let tetromino = generator.next();

        assert_ne!(tetromino, Tetromino::default())
    }
}
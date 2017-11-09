use super::tetromino::Tetromino;
use super::rand::{StdRng, Rng};

pub struct TetrominoGenerator {
    rng: StdRng
}

impl TetrominoGenerator{
    pub fn new(rng: StdRng) -> Self{
        TetrominoGenerator{
            rng
        }
    }

    pub fn next(&mut self) -> Option<Tetromino> {
        let random = self.rng.gen_range(0, 7);
        match random {
            0 => Some(Tetromino::I),
            1 => Some(Tetromino::J),
            2 => Some(Tetromino::L),
            3 => Some(Tetromino::Z),
            4 => Some(Tetromino::S),
            5 => Some(Tetromino::O),
            6 => Some(Tetromino::T),
            _ => None
        }
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
    use super::Tetromino;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn return_a_tetromino(){
        let rng = StdRng::from_seed(&[1]);
        let mut generator = TetrominoGenerator::new(rng);
        let tetromino = generator.next();

        assert_eq!(tetromino.unwrap(), Tetromino::O)
    }

//    fn never_generate_an_invalid_tetromino
}
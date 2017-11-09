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
            0 => Some(Tetromino::i()),
            1 => Some(Tetromino::j()),
            2 => Some(Tetromino::l()),
            3 => Some(Tetromino::z()),
            4 => Some(Tetromino::s()),
            5 => Some(Tetromino::o()),
            6 => Some(Tetromino::t()),
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

        assert_eq!(tetromino.unwrap(), Tetromino::o())
    }

//    fn never_generate_an_invalid_tetromino
}
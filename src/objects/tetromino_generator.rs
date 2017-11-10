use super::tetromino::Tetromino;
use ::rand::{StdRng, Rng};

#[derive(Copy, Clone)]
pub struct TetrominoGenerator {
    rng: StdRng
}

impl TetrominoGenerator{
    pub fn new(rng: StdRng) -> Self{
        TetrominoGenerator{
            rng
        }
    }

    pub fn next(&self) -> (TetrominoGenerator, Option<Tetromino>) {
        let mut rng = self.rng;
        let random = rng.gen_range(0, 7);
        let tetromino = match random {
            0 => Some(Tetromino::i()),
            1 => Some(Tetromino::j()),
            2 => Some(Tetromino::l()),
            3 => Some(Tetromino::z()),
            4 => Some(Tetromino::s()),
            5 => Some(Tetromino::o()),
            6 => Some(Tetromino::t()),
            _ => None
        };
        (TetrominoGenerator::new(self.rng), tetromino)
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
        let generator = TetrominoGenerator::new(rng);
        let (_generator, tetromino) = generator.next();

        assert_eq!(tetromino.unwrap(), Tetromino::o())
    }

    #[test]
    fn never_return_none(){
        let generator = TetrominoGenerator::default();
        for _ in 0..1000 {
            let (_generator, tetronimo) = generator.next();
            assert_ne!(tetronimo, None);
        }
    }
}
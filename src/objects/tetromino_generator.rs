use super::shape::Shape;
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

    pub fn next(&self) -> (TetrominoGenerator, Option<Shape>) {
        let mut rng = self.rng;
        let random = rng.gen_range(0, 7);
        let tetromino = match random {
            0 => Some(Shape::i()),
            1 => Some(Shape::j()),
            2 => Some(Shape::l()),
            3 => Some(Shape::z()),
            4 => Some(Shape::s()),
            5 => Some(Shape::o()),
            6 => Some(Shape::t()),
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
    use super::Shape;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn return_a_tetromino(){
        let rng = StdRng::from_seed(&[1]);
        let generator = TetrominoGenerator::new(rng);
        let (_generator, tetromino) = generator.next();

        assert_eq!(tetromino.unwrap(), Shape::o())
    }

    #[test]
    fn never_return_none(){
        let generator = TetrominoGenerator::default();
        for _ in 0..1000 {
            let (_generator, tetromino) = generator.next();
            assert_ne!(tetromino, None);
        }
    }
}
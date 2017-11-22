use rand::{StdRng, Rng};
use super::shape::*;
use super::Shape;
use super::{RandomTetrominoStream, TetrominoStream};

impl RandomTetrominoStream {
    pub fn new(rng: StdRng) -> Self {
        RandomTetrominoStream { rng }
    }
}

impl TetrominoStream for RandomTetrominoStream{
    fn next(&self) -> (RandomTetrominoStream, Option<Shape>) {
        let mut rng = self.rng;
        let random = rng.gen_range(0, 7);
        let tetromino = match random {
            0 => Some(I),
            1 => Some(J),
            2 => Some(L),
            3 => Some(Z),
            4 => Some(S),
            5 => Some(O),
            6 => Some(T),
            _ => panic!("must always provide a tetromino"),
        };
        (RandomTetrominoStream::new(self.rng), tetromino)
    }
}

impl Default for RandomTetrominoStream {
    fn default() -> Self {
        RandomTetrominoStream { rng: StdRng::new().unwrap() }
    }
}

#[cfg(test)]
mod should {
    use rand::{StdRng, SeedableRng};
    use pieces::shape::O;
    use super::*;

    #[test]
    fn return_a_tetromino() {
        let rng = StdRng::from_seed(&[1]);
        let generator = RandomTetrominoStream::new(rng);
        let (_generator, tetromino) = generator.next();

        assert_eq!(tetromino.unwrap(), O)
    }

    #[test]
    fn never_return_none() {
        let generator = RandomTetrominoStream::default();
        for _ in 0..1000 {
            let (_generator, tetromino) = generator.next();
            assert_ne!(tetromino, None);
        }
    }
}

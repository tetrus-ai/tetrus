use rand::{Rng, StdRng};
use super::shape::*;
use super::Shape;
use super::{RandomTetrominoStream, TetrominoStream};

impl RandomTetrominoStream {
    pub fn new(rng: StdRng) -> Self {
        RandomTetrominoStream { rng }
    }
}

impl TetrominoStream for RandomTetrominoStream {
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
        (RandomTetrominoStream::new(rng), tetromino)
    }
}

impl Default for RandomTetrominoStream {
    fn default() -> Self {
        RandomTetrominoStream {
            rng: StdRng::new().unwrap(),
        }
    }
}

#[cfg(test)]
mod should {
    use rand::{SeedableRng, StdRng};
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
    fn return_two_different_tetrominos_given_a_known_seed() {
        let rng = StdRng::from_seed(&[3]);
        let stream = RandomTetrominoStream::new(rng);
        let (stream, first_tetromino) = stream.next();
        let (_stream, second_tetromino) = stream.next();

        assert_eq!(first_tetromino.unwrap(), J);
        assert_eq!(second_tetromino.unwrap(), S)
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

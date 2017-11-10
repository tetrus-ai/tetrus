use super::tetromino::Tetromino;
use super::tetromino_generator::TetrominoGenerator;

#[derive(Clone, Copy)]
pub struct UpNext{
    pub first: Tetromino,
    pub second: Tetromino,
    generator: TetrominoGenerator
}

impl UpNext{
    pub fn new(generator: TetrominoGenerator) -> UpNext {
        let (generator, first) = generator.next();
        let (generator, second) = generator.next();
        UpNext {
            generator,
            first: first.unwrap(),
            second: second.unwrap(),
        }
    }

    pub fn next(&self) -> (UpNext, Tetromino) {
        let current = self.first;
        let (generator, next) = self.generator.next();
        let up_next = UpNext {
            first: self.second,
            second: next.unwrap(),
            generator: generator,
        };
        (up_next, current)
    }
}

#[cfg(test)]
mod should {
    use super::UpNext;
    use ::objects::tetromino_generator::TetrominoGenerator;
    use ::rand::{StdRng, SeedableRng};
    use ::objects::tetromino::Tetromino;

    #[test]
    fn set_new_current_to_old_first() {
        let generator = TetrominoGenerator::default();
        let up_next = UpNext::new(generator);

        let expected_current = up_next.first;
        let (_up_next, current) = up_next.next();

        assert_eq!(current, expected_current);
    }

    #[test]
    fn set_new_first_to_old_second() {
        let generator = TetrominoGenerator::default();
        let up_next = UpNext::new(generator);

        let expected_first = up_next.second;
        let (up_next, _current) = up_next.next();

        assert_eq!(up_next.first, expected_first);
    }

    #[test]
    fn set_new_second_to_expected_from_rng() {
        let generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let up_next = UpNext::new(generator);

        let (up_next, _current) = up_next.next();

        assert_eq!(up_next.second, Tetromino::t());
    }
}

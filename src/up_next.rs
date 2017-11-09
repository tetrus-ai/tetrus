use super::tetromino::Tetromino;
use super::tetromino_generator::TetrominoGenerator;

pub struct UpNext<'a>{
    pub first: Tetromino,
    pub second: Tetromino,
    generator: &'a mut TetrominoGenerator
}

impl<'a> UpNext<'a>{
    pub fn new(generator: &mut TetrominoGenerator) -> UpNext {
        let first = generator.next().unwrap();
        let second = generator.next().unwrap();
        UpNext {
            generator,
            first,
            second,
        }
    }

    pub fn next(&mut self) -> (UpNext, Tetromino) {
        let current = self.first;
        let up_next = UpNext {
            first: self.second,
            second: self.generator.next().unwrap(),
            generator: self.generator,
        };
        (up_next, current)
    }
}

#[cfg(test)]
mod should {
    use super::UpNext;
    use ::tetromino_generator::TetrominoGenerator;
    use ::rand::{StdRng, SeedableRng};
    use ::tetromino::Tetromino;

    #[test]
    fn set_new_current_to_old_first() {
        let mut generator = TetrominoGenerator::default();
        let mut in_play = UpNext::new(&mut generator);

        let expected_current = in_play.first;
        let (in_play, current) = in_play.next();

        assert_eq!(current, expected_current);
    }

    #[test]
    fn set_new_first_to_old_second() {
        let mut generator = TetrominoGenerator::default();
        let mut in_play = UpNext::new(&mut generator);

        let expected_first = in_play.second;
        let (in_play, current) = in_play.next();

        assert_eq!(in_play.first, expected_first);
    }

    #[test]
    fn set_new_second_to_expected_from_rng() {
        let mut generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let mut in_play = UpNext::new(&mut generator);

        let (in_play, current) = in_play.next();

        assert_eq!(in_play.second, Tetromino::t());
    }
}

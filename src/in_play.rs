use super::tetromino::Tetromino;
use super::tetromino_generator::TetrominoGenerator;

pub struct InPlay<'a>{
    pub current: Tetromino,
    pub first: Tetromino,
    pub second: Tetromino,
    generator: &'a mut TetrominoGenerator
}

impl<'a> InPlay<'a>{
    pub fn new(generator: &mut TetrominoGenerator) -> InPlay {
        let first = generator.next().unwrap();
        let second = generator.next().unwrap();
        let current = generator.next().unwrap();
        InPlay {
            generator,
            first,
            second,
            current,
        }
    }

    pub fn next(&mut self) -> InPlay {
        InPlay {
            current: self.first,
            first: self.second,
            second: self.generator.next().unwrap(),
            generator: self.generator,
        }
    }
}

#[cfg(test)]
mod should {
    use super::InPlay;
    use ::tetromino_generator::TetrominoGenerator;
    use ::rand::{StdRng, SeedableRng};
    use ::tetromino::Tetromino;

    fn set_new_current_to_old_first() {
        let mut generator = TetrominoGenerator::default();
        let mut in_play = InPlay::new(&mut generator);

        let expected_current = in_play.first;
        let in_play = in_play.next();

        assert_eq!(in_play.current, expected_current);
    }

    fn set_new_first_to_old_second() {
        let mut generator = TetrominoGenerator::default();
        let mut in_play = InPlay::new(&mut generator);

        let expected_first = in_play.second;
        let in_play = in_play.next();

        assert_eq!(in_play.first, expected_first);
    }

    fn set_new_second_to_expected_from_rng() {
        let mut rng = StdRng::from_seed(&[1]);
        let mut generator = TetrominoGenerator::new(rng);
        let mut in_play = InPlay::new(&mut generator);

        let in_play = in_play.next();

        assert_eq!(in_play.second, Tetromino::O);
    }
}

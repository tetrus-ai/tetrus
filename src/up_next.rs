use super::tetromino::Tetromino;
use super::tetromino_generator::TetrominoGenerator;

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

    pub fn next(&mut self) -> (UpNext, Tetromino) {
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

impl Copy for UpNext{}

impl Clone for UpNext{
    fn clone(&self) -> Self {
        UpNext{
            first: self.first,
            second: self.second,
            generator: self.generator
        }
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
        let mut in_play = UpNext::new(generator);

        let expected_current = in_play.first;
        let (in_play, current) = in_play.next();

        assert_eq!(current, expected_current);
    }

    #[test]
    fn set_new_first_to_old_second() {
        let mut generator = TetrominoGenerator::default();
        let mut in_play = UpNext::new(generator);

        let expected_first = in_play.second;
        let (in_play, current) = in_play.next();

        assert_eq!(in_play.first, expected_first);
    }

    #[test]
    fn set_new_second_to_expected_from_rng() {
        let mut generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let mut in_play = UpNext::new(generator);

        let (in_play, current) = in_play.next();

        assert_eq!(in_play.second, Tetromino::t());
    }
}

use super::UpNext;
use super::Shape;
use super::TetrominoGenerator;

impl UpNext {
    pub fn new(generator: TetrominoGenerator) -> UpNext {
        let (generator, first) = generator.next();
        let (generator, second) = generator.next();
        UpNext {
            generator,
            first: first.unwrap(),
            second: second.unwrap(),
        }
    }

    pub fn next(&self) -> (UpNext, Shape) {
        let current = self.first;
        let (generator, next) = self.generator.next();
        let up_next = UpNext {
            first: self.second,
            second: next.unwrap(),
            generator,
        };
        (up_next, current)
    }
}

#[cfg(test)]
mod should {
    use super::TetrominoGenerator;
    use super::UpNext;

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
}

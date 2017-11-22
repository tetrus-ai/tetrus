use super::RandomTetrominoBuffer;
use super::Shape;
use super::{RandomTetrominoStream, TetrominoStream};

impl RandomTetrominoBuffer {
    pub fn new(stream: RandomTetrominoStream) -> RandomTetrominoBuffer {
        let (stream, first) = stream.next();
        let (stream, second) = stream.next();
        RandomTetrominoBuffer {
            first: first.unwrap(),
            second: second.unwrap(),
            stream,
        }
    }

    pub fn next(&self) -> (RandomTetrominoBuffer, Shape) {
        let current = self.first;
        let (stream, next) = self.stream.next();
        let up_next = RandomTetrominoBuffer {
            first: self.second,
            second: next.unwrap(),
            stream,
        };
        (up_next, current)
    }
}

impl PartialEq for RandomTetrominoBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

#[cfg(test)]
mod should {
    use super::RandomTetrominoStream;
    use super::RandomTetrominoBuffer;

    #[test]
    fn set_new_current_to_old_first() {
        let stream = RandomTetrominoStream::default();
        let buffer = RandomTetrominoBuffer::new(stream);

        let expected_current = buffer.first;
        let (_buffer, current) = buffer.next();

        assert_eq!(current, expected_current);
    }

    #[test]
    fn set_new_first_to_old_second() {
        let stream = RandomTetrominoStream::default();
        let buffer = RandomTetrominoBuffer::new(stream);

        let expected_first = buffer.second;
        let (up_next, _buffer) = buffer.next();

        assert_eq!(up_next.first, expected_first);
    }

    #[test]
    fn be_equal_to_other_if_they_have_the_same_in_first_and_second() {
        let stream = RandomTetrominoStream::default();
        let first_buffer = RandomTetrominoBuffer::new(stream);
        let second_buffer = RandomTetrominoBuffer::new(stream);

        assert_eq!(first_buffer, second_buffer);
    }
}

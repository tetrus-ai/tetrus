use super::TetrominoBuffer;
use super::Shape;
use super::TetrominoStream;
use super::RandomTetrominoStream;

impl TetrominoBuffer<RandomTetrominoStream> {
    pub fn new(stream: ) -> TetrominoBuffer<RandomTetrominoStream> {
        let (stream, first) = stream.next();
        let (stream, second) = stream.next();
        TetrominoBuffer {
            first: first.unwrap(),
            second: second.unwrap(),
            stream,
        }
    }

    pub fn next(&self) -> (TetrominoBuffer<RandomTetrominoStream>, Shape) {
        let current = self.first;
        let (stream, next) = self.stream.next();
        let up_next = TetrominoBuffer {
            first: self.second,
            second: next.unwrap(),
            stream,
        };
        (up_next, current)
    }
}

impl PartialEq for TetrominoBuffer<RandomTetrominoStream> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first
    }
}

#[cfg(test)]
mod should {
    use super::TetrominoStream;
    use super::TetrominoBuffer;

    #[test]
    fn set_new_current_to_old_first() {
        let stream = TetrominoStream::default();
        let buffer = TetrominoBuffer::new(stream);

        let expected_current = buffer.first;
        let (_buffer, current) = buffer.next();

        assert_eq!(current, expected_current);
    }

    #[test]
    fn set_new_first_to_old_second() {
        let stream = TetrominoStream::default();
        let buffer = TetrominoBuffer::new(stream);

        let expected_first = buffer.second;
        let (up_next, _buffer) = buffer.next();

        assert_eq!(up_next.first, expected_first);
    }

    #[test]
    fn be_equal_to_other_if_they_have_the_same_in_first_and_second() {
        let stream = TetrominoStream::default();
        let first_buffer = TetrominoBuffer::new(stream);
        let second_buffer = TetrominoBuffer::new(stream);

        assert_eq!(first_buffer, second_buffer);
    }
}

use ::tetromino::Tetromino;
use ::tetromino_generator::TetrominoGenerator;

pub struct UpNext<'a>{
    pub first: Tetromino,
    pub second: Tetromino,
    generator: &'a mut TetrominoGenerator
}

impl<'a> UpNext<'a>{
    pub fn new(generator: &mut TetrominoGenerator) -> (UpNext, Tetromino){
        let first = generator.next().unwrap();
        let second = generator.next().unwrap();
        let current = generator.next().unwrap();
        let up_next = UpNext{
            generator,
            first,
            second,
        };
        (up_next, current)
    }

    pub fn next(&mut self) -> (UpNext, Tetromino) {
        let next = UpNext {
            first: self.second,
            second: self.generator.next().unwrap(),
            generator: self.generator,
        };
        (next, self.first)
    }
}

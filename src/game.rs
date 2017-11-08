use super::well::Well;
use super::tetromino::Tetromino;
use super::up_next::UpNext;
use super::tetromino_generator::GenerateTetromino;

pub struct Game<'a>{
    pub score: u32,
    pub well: Well,
    pub up_next: UpNext,
    pub in_play: Option<Tetromino>,
    generator: &'a GenerateTetromino,
}

impl<'a> Game<'a>{
    pub fn new(generator: &'a GenerateTetromino) -> Game {
        Game {
            score: 0,
            well: Well::empty(),
            up_next: UpNext::default(),
            generator,
            in_play: None,
        }
    }

    pub fn start(&mut self) {
        self.in_play = Some(Tetromino::default())
    }
}

#[cfg(test)]
mod constructor_should {
    use super::Game;
    use ::tetromino_generator::TetrominoGenerator;

    #[test]
    fn set_score_to_zero() {
        let generator = TetrominoGenerator::default();
        let game = Game::new(&generator);
        assert_eq!(game.score, 0);
    }
}

#[cfg(test)]
mod start_should {
    use super::Game;
    use ::tetromino_generator::TetrominoGenerator;
    use ::tetromino::Tetromino;

    #[test]
    fn put_next_into_play() {
        let generator = TetrominoGenerator::default();
        let mut game = Game::new(&generator);
        game.start();
        assert_eq!(game.in_play, Some(Tetromino::default()))
    }
}
use super::well::Well;
use super::tetromino::Tetromino;
use super::up_next::UpNext;
use super::tetromino_generator::GenerateTetromino;

pub struct Game<'a>{
    pub score: u32,
    pub well: Well,
    pub up_next: UpNext,
    generator: &'a GenerateTetromino
}

impl<'a> Game<'a>{
    pub fn new(generator: &'a GenerateTetromino) -> Game {
        Game {
            score: 0,
            well: Well::empty(),
            up_next: UpNext::default(),
            generator
        }
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
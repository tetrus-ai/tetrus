use super::well::Well;
use super::tetromino::Tetromino;
use super::up_next::UpNext;
use super::tetromino_generator::TetrominoGenerator;

pub struct Game<'a>{
    pub score: u32,
    pub well: Well,
    pub in_play: Tetromino,
    pub up_next: UpNext<'a>,
}

impl<'a> Game<'a>{
    pub fn new(generator: &mut TetrominoGenerator) -> Game {
        let (up_next, in_play) = UpNext::new(generator);
        Game {
            score: 0,
            well: Well::empty(),
            up_next,
            in_play,
        }
    }

    pub fn start(&mut self) -> Game {
        let (up_next, in_play) = self.up_next.next();
        Game {
            score: self.score,
            well: self.well,
            in_play,
            up_next,
        }
    }
}

#[cfg(test)]
mod new_game_should {
    use super::Game;
    use ::tetromino_generator::TetrominoGenerator;

    #[test]
    fn have_a_score_of_zero() {
        let mut generator = TetrominoGenerator::default();
        let game = Game::new(&mut generator);
        assert_eq!(game.score, 0);
    }
}

#[cfg(test)]
mod started_game_should {
    use super::Game;
    use ::tetromino_generator::TetrominoGenerator;
    use ::tetromino::Tetromino;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn have_a_tetromino_in_play() {
        let mut rng = StdRng::from_seed(&[1]);
        let mut generator = TetrominoGenerator::new(rng);
        let mut game = Game::new(&mut generator);
        game.start();
        assert_eq!(game.in_play, Tetromino::L)
    }
}
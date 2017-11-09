use super::well::Well;
use super::in_play::UpNext;
use super::tetromino::Tetromino;
use super::tetromino_generator::TetrominoGenerator;
use super::position::Position;
use super::current::Current;

pub struct Game<'a>{
    pub score: u32,
    pub well: Well,
    pub up_next: UpNext<'a>,
    pub current: Current
}

impl<'a> Game<'a>{
    pub fn new(generator: &mut TetrominoGenerator) -> Game {
        let current_tetromino = generator.next().unwrap();
        let current = Current::new(current_tetromino);
        let up_next = UpNext::new(generator);
        Game {
            score: 0,
            well: Well::empty(),
            up_next: up_next,
            current
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
    use ::position::Position;
    use ::tetromino::Tetromino;
    use ::tetromino_generator::TetrominoGenerator;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn have_a_tetromino_in_play() {
        let mut generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let mut game = Game::new(&mut generator);
        assert_eq!(game.current.tetromino, Tetromino::l());
        assert_eq!(game.current.position, Position::new(5, 2));
    }
}
use super::well::Well;
use super::up_next::UpNext;
use super::tetromino::Tetromino;
use super::tetromino_generator::TetrominoGenerator;
use super::position::Position;
use super::current::Current;

pub struct Game{
    pub score: u32,
    pub well: Well,
    pub up_next: UpNext,
    pub current: Current
}

impl Game{
    pub fn new(generator: TetrominoGenerator) -> Game {
        let (generator, current_tetromino) = generator.next();
        let current = Current::new(current_tetromino.unwrap());
        let up_next = UpNext::new(generator);
        Game {
            score: 0,
            well: Well::empty(),
            up_next: up_next,
            current
        }
    }

    pub fn tick(&self) -> Game {
        let current = self.current.dropByOne();
        Game {
            score: self.score,
            well: self.well,
            up_next: self.up_next,
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
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
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
        let generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let game = Game::new(generator);
        assert_eq!(game.current.tetromino, Tetromino::l());
        assert_eq!(game.current.position, Position::new(5, 2));
    }
}
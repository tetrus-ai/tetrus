use ::engine::command::Command;
use ::engine::current::Current;
use ::objects::tetromino_generator::TetrominoGenerator;
use ::objects::up_next::UpNext;
use ::objects::well::Well;

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
        let current = self.current.drop_by_one();
        Game {
            score: self.score,
            well: self.well,
            up_next: self.up_next,
            current
        }
    }

    pub fn issue_command(&self, command: Command) -> Game {
        let current = match command {
            Command::MoveLeft => self.current.move_left(),
            Command::MoveRight => Some(self.current.move_right())
        };

        match current {
            Some(x) => Game {
                score: self.score,
                well: self.well,
                up_next: self.up_next,
                current: x
            },
            None => Game {
                score: self.score,
                well: self.well,
                up_next: self.up_next,
                current: self.current
            }
        }
    }
}

#[cfg(test)]
mod new_game_should {
    use super::Game;
    use ::objects::tetromino_generator::TetrominoGenerator;

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
    use ::engine::position::Position;
    use ::objects::tetromino::Tetromino;
    use ::objects::tetromino_generator::TetrominoGenerator;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn have_a_tetromino_in_play() {
        let generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let game = Game::new(generator);
        assert_eq!(game.current.tetromino, Tetromino::l());
        assert_eq!(game.current.position, Position::new(5, 2));
    }
}

#[cfg(test)]
mod should {
    use super::Game;
    use ::engine::command::Command;
    use ::engine::position::Position;
    use ::objects::tetromino_generator::TetrominoGenerator;

    #[test]
    fn move_current_to_the_left_when_issued_a_move_left_command(){
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        let left_command = Command::MoveLeft;
        
        let game = game.issue_command(left_command);

        assert_eq!(game.current.position, Position::new(4, 2))
    }

    #[test]
    fn move_current_to_the_right_when_issued_a_move_right_command(){
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        let right_command = Command::MoveRight;
        
        let game = game.issue_command(right_command);

        assert_eq!(game.current.position, Position::new(6, 2))
    }

    #[test]
    fn not_make_an_illegal_move(){
        let generator = TetrominoGenerator::default();
        let mut game = Game::new(generator);
        game.current.position = Position::new(0, 0);
        
        let initial_position = game.current.position;

        let left_command = Command::MoveLeft;

        let game = game.issue_command(left_command);

        assert_eq!(game.current.position, initial_position);
    }
}
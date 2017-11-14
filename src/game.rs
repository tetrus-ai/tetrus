use ::engine::command::Command;
use ::engine::piece::Piece;
use ::objects::tetromino_generator::TetrominoGenerator;
use ::objects::up_next::UpNext;
use ::engine::piece_keeper::PieceKeeper;
use ::engine::position::Position;


pub const ORIGIN: Position = Position{ x:ORIGIN_X, y:ORIGIN_Y };
pub const ORIGIN_X: i8 = 0;
pub const ORIGIN_Y: i8 = 0;

pub const BOUNDARY_LEFT: i8 = -4;
pub const BOUNDARY_RIGHT: i8 = 4;
pub const BOUNDARY_BOTTOM: i8 = 20;

pub const MOVE_SPEED: i8 = 1;

pub struct Game{
    pub score: u32,
    pub up_next: UpNext,
    pub current: Piece,
    piece_keeper: PieceKeeper
}

impl Game{
    pub fn new(generator: TetrominoGenerator) -> Game {
        let (generator, shape) = generator.next();
        let current = Piece::new(shape.unwrap());
        let up_next = UpNext::new(generator);
        Game {
            score: 0,
            up_next: up_next,
            current,
            piece_keeper: PieceKeeper::default()
        }
    }

    pub fn tick(&self) -> Game {
        let current = self.piece_keeper.execute_command(Command::Drop, self.current);
        Game {
            score: self.score,
            up_next: self.up_next,
            current,
            piece_keeper: self.piece_keeper,
        }
    }

    pub fn issue_command(&self, command: Command) -> Game {
        let piece =  self.piece_keeper.execute_command(command, self.current);
        Game {
            score: self.score,
            up_next: self.up_next,
            current: piece,
            piece_keeper: self.piece_keeper,
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
    use super::*;
    use ::engine::position::Position;
    use ::objects::shape::Shape;
    use ::objects::tetromino_generator::TetrominoGenerator;
    use ::rand::{StdRng, SeedableRng};

    #[test]
    fn have_a_tetromino_in_play() {
        let generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let game = Game::new(generator);
        assert_eq!(game.current.shape, Shape::l());
        assert_eq!(game.current.position, Position::new(ORIGIN_X, ORIGIN_Y));
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use ::engine::command::Command;
    use ::engine::position::Position;
    use ::objects::tetromino_generator::TetrominoGenerator;

    #[test]
    fn move_current_to_the_left_when_issued_a_move_left_command(){
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        let left_command = Command::MoveLeft;
        
        let game = game.issue_command(left_command);

        assert_eq!(game.current.position, Position::new(ORIGIN_X - MOVE_SPEED, ORIGIN_Y))
    }

    #[test]
    fn move_current_to_the_right_when_issued_a_move_right_command(){
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        let right_command = Command::MoveRight;
        
        let game = game.issue_command(right_command);

        assert_eq!(game.current.position, Position::new(ORIGIN_X + MOVE_SPEED, ORIGIN_Y))
    }
}
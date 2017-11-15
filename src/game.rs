use super::pieces::{PlacedPiece, TetrominoGenerator};
use super::movements::{PieceKeeper, Command};
use super::pieces::{UpNext, Position};

pub const ORIGIN: Position = Position {
    x: ORIGIN_X,
    y: ORIGIN_Y,
};
pub const ORIGIN_X: i8 = 0;
pub const ORIGIN_Y: i8 = 0;

pub const BOUNDARY_LEFT: i8 = -4;
pub const BOUNDARY_RIGHT: i8 = 4;
pub const BOUNDARY_BOTTOM: i8 = 20;

pub const MOVE_SPEED: i8 = 1;

pub struct Game {
    pub score: u32,
    pub up_next: UpNext,
    pub current_piece: PlacedPiece,
    piece_keeper: PieceKeeper,
}

impl Game {
    pub fn new(generator: TetrominoGenerator) -> Game {
        let (generator, shape) = generator.next();
        let initial_piece = PlacedPiece::at_origin_with_shape(shape.unwrap());
        let up_next = UpNext::new(generator);
        Game {
            score: 0,
            current_piece: initial_piece,
            piece_keeper: PieceKeeper::default(),
            up_next,
        }
    }

    pub fn issue_command(&self, command: Command) -> Game {
        let moved_piece = self.piece_keeper.execute(command, self.current_piece);
        Game {
            score: self.score,
            up_next: self.up_next,
            current_piece: moved_piece,
            piece_keeper: self.piece_keeper,
        }
    }
}

#[cfg(test)]
mod new_game_should {
    use super::Game;
    use pieces::TetrominoGenerator;

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
    use pieces::{Position, TetrominoGenerator};
    use rand::{StdRng, SeedableRng};

    #[test]
    fn have_a_tetromino_in_play() {
        let generator = TetrominoGenerator::new(StdRng::from_seed(&[1]));
        let game = Game::new(generator);
        assert_eq!(
            game.current_piece.position,
            Position::new(ORIGIN_X, ORIGIN_Y)
        );
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn move_current_to_the_left_when_issued_a_move_left_command() {
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        let left_command = Command::MoveLeft;

        let game = game.issue_command(left_command);

        assert_eq!(
            game.current_piece.position,
            Position::new(ORIGIN_X - MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_the_right_when_issued_a_move_right_command() {
        let generator = TetrominoGenerator::default();
        let game = Game::new(generator);
        let right_command = Command::MoveRight;

        let game = game.issue_command(right_command);

        assert_eq!(
            game.current_piece.position,
            Position::new(ORIGIN_X + MOVE_SPEED, ORIGIN_Y)
        )
    }
}

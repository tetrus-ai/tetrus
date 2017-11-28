use pieces::{PlacedPiece, TetrominoStream, RandomTetrominoStream};
use movements::{PieceKeeper, Command, MotionController};
use pieces::{RandomTetrominoBuffer, Position};
use super::GameState;

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

impl GameState {
    pub fn new(stream: RandomTetrominoStream) -> GameState {
        let (stream, shape) = stream.next();
        let initial_piece = PlacedPiece::at_origin_with_shape(shape.unwrap());
        let next_pieces = RandomTetrominoBuffer::new(stream);
        GameState {
            score: 0,
            current_piece: initial_piece,
            piece_keeper: PieceKeeper::default(),
            next_pieces,
        }
    }

    pub fn issue_command(&self, command: Command) -> GameState {
        let moved_piece = self.piece_keeper.move_piece(command, self.current_piece);
        GameState {
            score: self.score,
            next_pieces: self.next_pieces,
            current_piece: moved_piece,
            piece_keeper: self.piece_keeper,
        }
    }
}

impl PartialEq for GameState{
    fn eq(&self, other: &Self) -> bool{
        self.score == other.score 
        && self.current_piece == other.current_piece 
        && self.next_pieces == other.next_pieces 
    }
}

#[cfg(test)]
mod new_game_should {
    use super::GameState;
    use pieces::RandomTetrominoStream;

    #[test]
    fn have_a_score_of_zero() {
        let generator = RandomTetrominoStream::default();
        let game = GameState::new(generator);
        assert_eq!(game.score, 0);
    }
}

#[cfg(test)]
mod started_game_should {
    use super::*;
    use pieces::{Position, RandomTetrominoStream};
    use rand::{StdRng, SeedableRng};

    #[test]
    fn have_a_tetromino_in_play() {
        let generator = RandomTetrominoStream::new(StdRng::from_seed(&[1]));
        let game = GameState::new(generator);
        assert_eq!(
            game.current_piece.position,
            Position::new(ORIGIN_X, ORIGIN_Y)
        );
    }
}

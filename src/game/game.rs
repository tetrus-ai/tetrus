use movements::Command;
use movements::MotionController;
use super::Game;
use super::PlayAreaSize;
use pieces::RandomTetrominoServer;
use pieces::PlacedPiece;


impl<'a> Game<'a> {
    pub fn default_ruleset(size: PlayAreaSize, buffer: RandomTetrominoServer, motion_controller: &'a MotionController) -> Self {
        let (next_pieces, current_shape) = buffer.next();
        Game {
            next_pieces,
            current: PlacedPiece::at_origin_with_shape(current_shape),
            motion_controller
        }
    }

    pub fn issue_command(&self, command: Command) -> Game {
        let current = self.motion_controller.move_piece(command, self.current);
        Game {
            next_pieces: self.next_pieces,
            current,
            motion_controller: self.motion_controller
        }
    }
}

impl<'a> PartialEq for Game<'a> {
    fn eq(&self, other: &Self) -> bool{
        self.current == other.current
        && self.next_pieces == other.next_pieces
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use pieces::{Position, PlacedPiece, Shape};
    use pieces::shape::*;
    use pieces::*;
    use movements::Command::*;

    use double;

    impl PlacedPiece {
        fn at_position_with_shape(position: Position, shape: Shape) -> PlacedPiece {
            PlacedPiece {
                position, shape
            }
        }
    }

    #[test]
    fn initialise_with_current_as_first_from_buffer() {
        let (buffer, some_size, motion_controller) = setup();
        let expected_current = buffer.first;

        let game = Game::default_ruleset(some_size, buffer, &motion_controller);

        assert_eq!(game.current.shape, expected_current);
    }

    #[test]
    fn move_the_piece_on_a_command() {
        let (buffer, some_size, motion_controller) = setup();
        let initial_game = Game::default_ruleset(some_size, buffer, &motion_controller);
        let piece = PlacedPiece::at_position_with_shape(Position::new(-1,0),Z);
        let final_piece = PlacedPiece::at_position_with_shape(Position::new(-2,0),Z);
        motion_controller.move_piece.return_value(piece);
        motion_controller.move_piece.return_value_for((MoveLeft, piece), final_piece);

        let new_game = initial_game.issue_command(MoveLeft);
        let final_game = new_game.issue_command(MoveLeft);

        assert!(motion_controller.move_piece.called_with((MoveLeft, initial_game.current)));
        assert!(motion_controller.move_piece.called_with((MoveLeft, piece)));
        assert_eq!(new_game.current, piece);
        assert_eq!(final_game.current, final_piece);
    }

    fn setup() -> (RandomTetrominoServer, PlayAreaSize, FakeMotionController) {
        (get_buffer(), get_play_area_size(), get_motion_controller())
    }

    fn get_buffer() -> RandomTetrominoServer {
        RandomTetrominoServer::new(RandomTetrominoStream::default())
    }

    fn get_play_area_size() -> PlayAreaSize {
        PlayAreaSize::with_width_and_height(0, 0)
    }

    fn get_motion_controller() -> FakeMotionController {
        FakeMotionController::default()
    }

    mock_trait!(
        FakeMotionController,
        move_piece(Command, PlacedPiece) -> PlacedPiece);

    impl MotionController for FakeMotionController{
        mock_method!(move_piece(&self, command: Command, piece: PlacedPiece) -> PlacedPiece);
    }

    impl Default for PlacedPiece{
        fn default() -> Self{
            PlacedPiece::at_origin_with_shape(I)
        }
    }
}

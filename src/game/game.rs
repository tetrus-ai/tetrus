use movements::Command;
use movements::MotionController;
use super::Game;
use super::PlayAreaSize;
use super::GameState;
use pieces::RandomTetrominoBuffer;
use pieces::PlacedPiece;
use pieces::RandomTetrominoStream;


impl<'a> Game<'a> {
    pub fn default_ruleset(size: PlayAreaSize, buffer: RandomTetrominoBuffer, motion_controller: &'a MotionController) -> Self {
        let (next_pieces, current) = buffer.next();
        Game {
            next_pieces,
            current: PlacedPiece::at_origin_with_shape(current),
            game_state: GameState::new(RandomTetrominoStream::default()),
            motion_controller
        }
    }

    pub fn issue_command(&self, command: Command) -> Game {
        let current = self.motion_controller.move_piece(command, self.current);
        Game {
            next_pieces: self.next_pieces,
            game_state: self.game_state,
            motion_controller: self.motion_controller,
            current,
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use super::super::game_state::*;
    use pieces::Position;

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
        let game = Game::default_ruleset(some_size, buffer, &motion_controller);
        // see if the controller is called properly

        // did the game state change ?
    }

    #[derive(Default)]
    struct FakeMotionController {}

    impl MotionController for FakeMotionController {
        fn move_piece(&self, command: Command, piece: PlacedPiece) -> PlacedPiece {
            piece
        }
    }

    fn setup() -> (RandomTetrominoBuffer, PlayAreaSize, FakeMotionController) {
        (get_buffer(), get_play_area_size(), get_motion_controller())
    }

    fn get_buffer() -> RandomTetrominoBuffer {
        RandomTetrominoBuffer::new(RandomTetrominoStream::default())
    }

    fn get_play_area_size() -> PlayAreaSize {
        PlayAreaSize::with_width_and_height(0, 0)
    }

    fn get_motion_controller() -> FakeMotionController {
        FakeMotionController::default()
    }
}

use movements::Command;
use movements::MotionController;
use super::Game;
use super::PlayAreaSize;
use super::GameState;
use pieces::RandomTetrominoBuffer;
use pieces::PlacedPiece;
use pieces::RandomTetrominoStream;


impl Game {
    pub fn default_ruleset(size: PlayAreaSize, buffer: RandomTetrominoBuffer, motion_controller: &MotionController) -> Self {
        let (next_pieces, current) = buffer.next();
        Game {
            next_pieces,
            current: PlacedPiece::at_origin_with_shape(current),
            game_state: GameState::new(RandomTetrominoStream::default()),
        }
    }

    pub fn issue_command(&mut self, command: Command) {}
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn initialise_with_current_as_first_from_buffer() {
        let buffer = get_buffer();
        let some_size = PlayAreaSize::with_width_and_height(1, 1);
        let expected_current = buffer.first;
        let motion_controller = FakeMotionController::default();
        let game = Game::default_ruleset(some_size, buffer, &motion_controller);

        assert_eq!(game.current.shape, expected_current);
    }

    #[test]
    fn move_the_piece_on_a_command() {
        let some_size = PlayAreaSize::with_width_and_height(0, 0);
        let buffer = RandomTetrominoBuffer::new(RandomTetrominoStream::default());
        let motion_controller = FakeMotionController::default();
        let game = Game::default_ruleset(some_size, buffer, &motion_controller);
        // mock the controller
        // see if the controller is called properly
            lol here
        // did the game state change ?
    }

    #[derive(Default)]
    struct FakeMotionController {}
    impl MotionController for FakeMotionController{}

    fn get_buffer() -> RandomTetrominoBuffer {
        RandomTetrominoBuffer::new(RandomTetrominoStream::default())
    }
}

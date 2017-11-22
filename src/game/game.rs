use movements::Command;
use super::Game;
use super::PlayAreaSize;
use super::GameState;
use pieces::RandomTetrominoBuffer;
use pieces::PlacedPiece;
use pieces::RandomTetrominoStream;

impl Game {
    pub fn default_ruleset(size: PlayAreaSize, buffer: RandomTetrominoBuffer) -> Self {
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

        let game = Game::default_ruleset(some_size, buffer);

        assert_eq!(game.current.shape, expected_current);
    }

    #[test]
    fn move_the_piece_on_a_command() {

        // mock the controller
        // see if the controller is called properly

        // did the game state change ?
    }

    fn get_buffer() -> RandomTetrominoBuffer {
        RandomTetrominoBuffer::new(RandomTetrominoStream::default())
    }
}

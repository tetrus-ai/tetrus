use movements::Command;
use super::Game;
use super::PlayAreaSize;
use super::GameState;
use pieces::TetrominoBuffer;
use pieces::RandomTetrominoStream;

impl Game {
    pub fn default_ruleset(size: PlayAreaSize) -> Self {
        Game {
            next_pieces: TetrominoBuffer::new(RandomTetrominoStream::default()),
            game_state: GameState::new(RandomTetrominoStream::default())
        }
    }

    pub fn issue_command(&mut self, command: Command) {
        let (next, current) = self.next_pieces.next();
        self.next_pieces = next;
    }
}

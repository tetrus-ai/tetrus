use movements::Command;
use super::Game;
use super::PlayAreaSize;
use pieces::TetrominoBuffer;
use pieces::TetrominoStream;

impl Game {
    pub fn default_ruleset(size: PlayAreaSize) -> Self {
        Game {
            next_pieces: TetrominoBuffer::new(TetrominoStream::default())
        }
    }

    pub fn issue_command(&self, command: Command) {}
}

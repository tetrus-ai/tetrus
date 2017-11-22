use movements::Command;
use super::Game;
use super::PlayAreaSize;
use pieces::TetrominoBuffer;
use pieces::RandomTetrominoStream;

impl Game {
    pub fn default_ruleset(size: PlayAreaSize) -> Self {
        Game {
            next_pieces: TetrominoBuffer::new(RandomTetrominoStream::default())
        }
    }

    pub fn issue_command(&self, command: Command) {}
}

use super::pieces::PlacedPiece;
use super::movements::PieceKeeper;
use super::pieces::RandomTetrominoBuffer;
use super::pieces::RandomTetrominoStream;

pub mod game;
pub mod game_state;
pub mod play_area_size;

pub struct Game {
    pub next_pieces: RandomTetrominoBuffer,
    pub game_state: GameState,
    pub current: PlacedPiece
}

pub struct GameState {
    pub score: u32,
    pub next_pieces: RandomTetrominoBuffer,
    pub current_piece: PlacedPiece,
    piece_keeper: PieceKeeper,
}

pub struct PlayAreaSize {
    width: u8,
    height: u8,
}

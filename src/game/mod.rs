use super::pieces::PlacedPiece;
use super::movements::PieceKeeper;
use super::pieces::TetrominoBuffer;
use super::pieces::RandomTetrominoStream;

pub mod game;
pub mod game_state;
pub mod play_area_size;

pub struct Game {
    pub next_pieces: TetrominoBuffer<RandomTetrominoStream>,
    pub game_state: GameState
}

pub struct GameState {
    pub score: u32,
    pub next_pieces: TetrominoBuffer<RandomTetrominoStream>,
    pub current_piece: PlacedPiece,
    piece_keeper: PieceKeeper,
}

pub struct PlayAreaSize {
    width: u8,
    height: u8,
}

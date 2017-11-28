use super::pieces::PlacedPiece;
use super::movements::PieceKeeper;
use super::pieces::RandomTetrominoBuffer;
use super::movements::MotionController;

pub mod game;
pub mod state;
pub mod play_area_size;

#[derive(Debug)]
pub struct Game<'a> {
    pub next_pieces: RandomTetrominoBuffer,
    pub state: GameState,
    pub current: PlacedPiece,
    pub motion_controller: &'a MotionController
}

#[derive(Clone, Copy, Debug)]
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

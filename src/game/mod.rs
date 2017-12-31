use super::pieces::PlacedPiece;
use super::movements::MotionController;
use super::pieces::RandomTetrominoServer;

pub mod game;
pub mod play_area_size;

#[derive(Debug)]
pub struct Game<'a> {
    pub next_pieces: RandomTetrominoServer,
    pub current: PlacedPiece,
    pub motion_controller: &'a MotionController
}

#[derive(Clone, Copy, Debug)]
pub struct GameState {
    pub score: u32,
    pub next_pieces: RandomTetrominoServer,
    pub current_piece: PlacedPiece,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayAreaSize {
    pub width: u8,
    pub height: u8,
}

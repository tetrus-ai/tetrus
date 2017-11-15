use super::pieces::PlacedPiece;
use super::movements::PieceKeeper;
use super::pieces::UpNext;


pub mod game;
pub mod game_state;
pub mod play_area_size;

pub struct Game {}

pub struct GameState {
    pub score: u32,
    pub up_next: UpNext,
    pub current_piece: PlacedPiece,
    piece_keeper: PieceKeeper,
}

pub struct PlayAreaSize {
    width: u8,
    height: u8,
}

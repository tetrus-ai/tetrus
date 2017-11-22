use pieces::PlacedPiece;

mod piece_keeper;

#[derive(Clone, Copy)]
pub enum Command {
    MoveLeft,
    MoveToLeftEdge,
    MoveRight,
    MoveToRightEdge,
    Drop,
    DropToBottom,
}

#[derive(Clone, Copy, Default)]
pub struct PieceKeeper {}

pub trait MotionController {
    fn move_piece(&self, command: Command, piece: PlacedPiece) -> PlacedPiece;
}

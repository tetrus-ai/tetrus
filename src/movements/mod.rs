use pieces::PlacedPiece;

mod piece_keeper;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    MoveLeft,
    MoveToLeftEdge,
    MoveRight,
    MoveToRightEdge,
    Drop,
    DropToBottom,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PieceKeeper {}

pub trait MotionController: ::std::fmt::Debug {
    fn move_piece(&self, command: Command, piece: PlacedPiece) -> PlacedPiece;
}

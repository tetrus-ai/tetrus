use pieces::PlacedPiece;

mod motion_control;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    MoveLeft,
    MoveToLeftEdge,
    MoveRight,
    MoveToRightEdge,
    Drop,
    DropToBottom,
}

#[derive(Debug, Default)]
pub struct DefaultMotionController{}

pub trait MotionController: ::std::fmt::Debug {
    fn move_piece(&self, command: Command, piece: PlacedPiece) -> PlacedPiece;
}

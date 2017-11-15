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

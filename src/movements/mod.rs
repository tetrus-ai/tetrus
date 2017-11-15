mod piece_keeper;

#[derive(Clone, Copy)]
pub enum Command {
    MoveLeft,
    MoveRight,
    Drop,
}

#[derive(Clone, Copy, Default)]
pub struct PieceKeeper {}

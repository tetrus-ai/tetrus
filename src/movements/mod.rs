mod piece_keeper;
mod banana_motion_controller;

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

pub trait MotionController{}

#[derive(Default)]
pub struct BananaMotionController {}

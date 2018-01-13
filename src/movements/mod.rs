use pieces::PlacedPiece;
use rules::RuleSet;

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

#[derive(Debug)]
pub struct DefaultMotionController {
    ruleset: RuleSet,
}

pub trait MotionController: ::std::fmt::Debug {
    fn move_piece(&self, command: Command, piece: PlacedPiece) -> PlacedPiece;
    fn can_be_moved_further(&self, piece: PlacedPiece) -> bool;
}

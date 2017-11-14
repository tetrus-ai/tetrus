use super::command::Command;
use super::piece::Piece;
use super::rules::out_of_bounds::*;

#[derive(Clone, Copy, Default)]
pub struct PieceKeeper{}

impl PieceKeeper{
    /* executes commands against a ruleset
       e.g. out_of_bounds::outside_of_left_boundary(&attempted_move)
    */
    pub fn execute_command(&self, command: Command, piece: Piece) -> Piece{
        let attempted_move = match command {
            Command::MoveLeft => piece.move_left(),
            Command::MoveRight => piece.move_right(),
            Command::Drop => piece.drop_by_one()
        };

        let mut is_allowed = true;
        is_allowed &= !outside_of_left_boundary(&attempted_move);
        is_allowed &= !outside_of_right_boundary(&attempted_move);
        
        match is_allowed {
            true => attempted_move,
            false => piece
        }
    }
}
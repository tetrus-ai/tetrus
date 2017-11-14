use super::command::Command;
use super::piece::Piece;
use super::rules::out_of_bounds::*;

#[derive(Clone, Copy, Default)]
pub struct PieceKeeper{}

impl PieceKeeper{
    pub fn execute_command(&self, command: Command, piece: Piece) -> Piece{
        let attempted_move = match command {
            Command::MoveLeft => piece.move_left(),
            Command::MoveRight => piece.move_right(),
            Command::Drop => piece.drop_by_one()
        };

        let mut is_allowed = true;
        is_allowed &= respects_rule(&outside_of_left_boundary, &attempted_move);
        is_allowed &= respects_rule(&outside_of_right_boundary, &attempted_move);
        
        #[allow(match_bool)]
        match is_allowed {
            true => attempted_move,
            false => piece
        }
    }
}

pub fn respects_rule(rule: &Fn(&Piece) -> RuleEvaluationResult, piece: &Piece) -> bool {
    match rule(piece){
        RuleEvaluationResult::Respected => true,
        RuleEvaluationResult::Violated => false
    }
}
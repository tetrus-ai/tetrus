use ::engine::piece::Piece;
use ::game::{BOUNDARY_LEFT, BOUNDARY_RIGHT};

pub enum RuleEvaluationResult {
    Violated,
    Respected
}

pub fn outside_of_left_boundary(&piece: &Piece) -> RuleEvaluationResult {
    let (x,_) = piece.position.into();
    if x < BOUNDARY_LEFT { RuleEvaluationResult::Violated } else { RuleEvaluationResult::Respected }
}

pub fn outside_of_right_boundary(&piece: &Piece) -> RuleEvaluationResult {
    let (x,_) = piece.position.into();
    if x > BOUNDARY_RIGHT { RuleEvaluationResult::Violated } else { RuleEvaluationResult::Respected }
}
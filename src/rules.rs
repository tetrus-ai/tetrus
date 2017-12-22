use pieces::{BOUNDARY_LEFT, BOUNDARY_RIGHT, BOUNDARY_BOTTOM};
use super::pieces::PlacedPiece;

#[derive(Debug, PartialEq)]
pub enum RuleEvaluationResult {
    Violated,
    Respected,
}

pub fn right_of_left_boundary(&piece: &PlacedPiece) -> RuleEvaluationResult {
    let (x, _) = piece.position.into();
    if x < BOUNDARY_LEFT {
        RuleEvaluationResult::Violated
    } else {
        RuleEvaluationResult::Respected
    }
}

pub fn left_of_right_boundary(&piece: &PlacedPiece) -> RuleEvaluationResult {
    let (x, _) = piece.position.into();
    if x > BOUNDARY_RIGHT {
        RuleEvaluationResult::Violated
    } else {
        RuleEvaluationResult::Respected
    }
}

pub fn above_bottom(&piece: &PlacedPiece) -> RuleEvaluationResult {
    let (_, y) = piece.position.into();
    if y > BOUNDARY_BOTTOM {
        RuleEvaluationResult::Violated
    } else {
        RuleEvaluationResult::Respected
    }
}

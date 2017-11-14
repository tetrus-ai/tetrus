use engine::piece::PlacedPiece;
use game::{BOUNDARY_LEFT, BOUNDARY_RIGHT};

pub enum RuleEvaluationResult {
    Violated,
    Respected,
}

pub fn outside_of_left_boundary(&piece: &PlacedPiece) -> RuleEvaluationResult {
    let (x, _) = piece.position.into();
    if x < BOUNDARY_LEFT {
        RuleEvaluationResult::Violated
    } else {
        RuleEvaluationResult::Respected
    }
}

pub fn outside_of_right_boundary(&piece: &PlacedPiece) -> RuleEvaluationResult {
    let (x, _) = piece.position.into();
    if x > BOUNDARY_RIGHT {
        RuleEvaluationResult::Violated
    } else {
        RuleEvaluationResult::Respected
    }
}

#[cfg(test)]
mod should{
    #[test]
    fn allow_in_bounds_placement(){}
}
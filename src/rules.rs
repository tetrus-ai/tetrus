use super::pieces::PlacedPiece;
use game::PlayAreaSize;

#[derive(Debug, PartialEq)]
pub enum RuleEvaluationResult {
    Violated,
    Respected,
}

pub struct RuleSet {
    play_area_size: PlayAreaSize
}

impl Default for RuleSet {
    fn default() -> Self {
       RuleSet {
           play_area_size = PlayAreaSize {
                height = 20,
                width = 5
           }
       }
    }
}

impl RuleSet {
    pub fn right_of_left_boundary(&self, &piece: &PlacedPiece) -> RuleEvaluationResult {
        let (x, _) = piece.position.into();
        if x < BOUNDARY_LEFT {
            RuleEvaluationResult::Violated
        } else {
            RuleEvaluationResult::Respected
        }
    }

    pub fn left_of_right_boundary(&self, &piece: &PlacedPiece) -> RuleEvaluationResult {
        let (x, _) = piece.position.into();
        if x > BOUNDARY_RIGHT {
            RuleEvaluationResult::Violated
        } else {
            RuleEvaluationResult::Respected
        }
    }

    pub fn above_bottom(&self, &piece: &PlacedPiece) -> RuleEvaluationResult {
        let (_, y) = piece.position.into();
        if y > BOUNDARY_BOTTOM {
            RuleEvaluationResult::Violated
        } else {
            RuleEvaluationResult::Respected
        }
    }
}

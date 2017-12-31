use super::pieces::PlacedPiece;
use game::PlayAreaSize;

#[derive(Debug, PartialEq)]
pub enum RuleEvaluationResult {
    Violated,
    Respected,
}

#[derive(Clone, Copy, Debug)]
pub struct RuleSet {
    pub play_area_size: PlayAreaSize,
    pub boundary_left: i8,
    pub boundary_right: i8,
    pub boundary_bottom: i8,
}

impl Default for RuleSet {
    fn default() -> Self {
        let width: i8 = 5;
        let height: i8 = 20;
        RuleSet {
            play_area_size: PlayAreaSize::with_width_and_height(width as u8, height as u8),
            boundary_left: -1 * (width / 2 - 1),
            boundary_right: 1 * (width / 2 - 1),
            boundary_bottom: height,
        }
    }
}

impl RuleSet {
    pub fn with_play_area_size(play_area_size: PlayAreaSize) -> Self {
        RuleSet {
            play_area_size,
            boundary_left: -1 * (play_area_size.width / 2 - 1) as i8,
            boundary_right: 1 * (play_area_size.width / 2 - 1) as i8,
            boundary_bottom: play_area_size.height as i8,
        }
    }

    pub fn right_of_left_boundary(&self, &piece: &PlacedPiece) -> RuleEvaluationResult {
        let (x, _) = piece.position.into();
        if x < self.boundary_left {
            RuleEvaluationResult::Violated
        } else {
            RuleEvaluationResult::Respected
        }
    }

    pub fn left_of_right_boundary(&self, &piece: &PlacedPiece) -> RuleEvaluationResult {
        let (x, _) = piece.position.into();
        if x > self.boundary_right {
            RuleEvaluationResult::Violated
        } else {
            RuleEvaluationResult::Respected
        }
    }

    pub fn above_bottom(&self, &piece: &PlacedPiece) -> RuleEvaluationResult {
        let (_, y) = piece.position.into();
        if y > self.boundary_bottom {
            RuleEvaluationResult::Violated
        } else {
            RuleEvaluationResult::Respected
        }
    }
}

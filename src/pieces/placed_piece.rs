use super::PlacedPiece;
use super::Shape;
use ::rules::*;
use super::*;

impl PlacedPiece {
    pub const fn at_origin_with_shape(shape: Shape) -> Self {
        PlacedPiece {
            shape,
            position: ORIGIN,
        }
    }

    fn with_new_position_respecting_rule(&self, position: Position, respecting_rule: &Fn(&PlacedPiece) -> RuleEvaluationResult) -> Self {
        let new_piece = PlacedPiece {
            shape: self.shape,
            position,
        };
        match respecting_rule(&new_piece) {
            RuleEvaluationResult::Respected => new_piece,
            RuleEvaluationResult::Violated => *self
        }
    }

    pub fn drop_by_one(&self, respecting_rule: &Fn(&PlacedPiece) -> RuleEvaluationResult) -> Self {
        let position = self.position.add_to_y(MOVE_SPEED);
        self.with_new_position_respecting_rule(position, respecting_rule)
    }

    pub fn move_left(&self, respecting_rule: &Fn(&PlacedPiece) -> RuleEvaluationResult) -> Self {
        let position = self.position.subtract_from_x(MOVE_SPEED);
        self.with_new_position_respecting_rule(position, respecting_rule)
    }

    pub fn move_right(&self, respecting_rule: &Fn(&PlacedPiece) -> RuleEvaluationResult) -> Self {
        let position = self.position.add_to_x(MOVE_SPEED);
        self.with_new_position_respecting_rule(position, respecting_rule)
    }
}

#[cfg(test)]
mod should {
    use pieces::MOVE_SPEED;
    use pieces::shape::I;
    use super::*;

    #[test]
    fn decrease_x_by_one_when_moved_left() {
        let current = PlacedPiece::at_origin_with_shape(I);
        let initial_x = current.position.x;
        let initial_y = current.position.y;
        let expected_x = initial_x - MOVE_SPEED;
        let current = current.move_left(&always_allow_move);

        assert_eq!(current.position.x, expected_x);
        assert_eq!(current.position.y, initial_y);
    }

    #[test]
    fn increase_x_by_one_when_moved_right() {
        let current = PlacedPiece::at_origin_with_shape(I);
        let initial_x = current.position.x;
        let initial_y = current.position.y;
        let expected_x = initial_x + MOVE_SPEED;
        let current = current.move_right(&always_allow_move);

        assert_eq!(current.position.x, expected_x);
        assert_eq!(current.position.y, initial_y);
    }

    fn always_allow_move(&piece: &PlacedPiece) -> RuleEvaluationResult {
        RuleEvaluationResult::Respected
    }
}

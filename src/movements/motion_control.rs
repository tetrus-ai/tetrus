use super::Command;
use super::MotionController;
use super::DefaultMotionController;

use rules::*;
use pieces::PlacedPiece;

impl DefaultMotionController {
    pub fn new(ruleset: RuleSet) -> Self {
        DefaultMotionController { ruleset }
    }

    fn move_respecting_rule(
        &self,
        piece_to_move: PlacedPiece,
        move_to_apply: &Fn(&PlacedPiece) -> PlacedPiece,
        rule: &Fn(&RuleSet, &PlacedPiece) -> RuleEvaluationResult,
    ) -> PlacedPiece {
        let new_piece = move_to_apply(&piece_to_move);
        match rule(&self.ruleset, &new_piece) {
            RuleEvaluationResult::Respected => new_piece,
            RuleEvaluationResult::Violated => piece_to_move,
        }
    }

    fn keep_moving_until_rule_no_longer_respected(
        &self,
        piece_to_move: PlacedPiece,
        move_to_apply: &Fn(&PlacedPiece) -> PlacedPiece,
        rule: &Fn(&RuleSet, &PlacedPiece) -> RuleEvaluationResult,
    ) -> PlacedPiece {
        let new_piece = move_to_apply(&piece_to_move);
        match rule(&self.ruleset, &new_piece) {
            RuleEvaluationResult::Respected => {
                self.keep_moving_until_rule_no_longer_respected(new_piece, move_to_apply, rule)
            }
            RuleEvaluationResult::Violated => piece_to_move,
        }
    }
}

impl MotionController for DefaultMotionController {
    fn move_piece(&self, command: Command, piece_to_move: PlacedPiece) -> PlacedPiece {
        match command {
            Command::MoveLeft => self.move_respecting_rule(
                piece_to_move,
                &PlacedPiece::move_left,
                &RuleSet::right_of_left_boundary,
            ),
            Command::MoveToLeftEdge => self.keep_moving_until_rule_no_longer_respected(
                piece_to_move,
                &PlacedPiece::move_left,
                &RuleSet::right_of_left_boundary,
            ),
            Command::MoveRight => self.move_respecting_rule(
                piece_to_move,
                &PlacedPiece::move_right,
                &RuleSet::left_of_right_boundary,
            ),
            Command::MoveToRightEdge => self.keep_moving_until_rule_no_longer_respected(
                piece_to_move,
                &PlacedPiece::move_right,
                &RuleSet::left_of_right_boundary,
            ),
            Command::Drop => self.move_respecting_rule(
                piece_to_move,
                &PlacedPiece::drop_by_one,
                &RuleSet::above_bottom,
            ),
            Command::DropToBottom => self.keep_moving_until_rule_no_longer_respected(
                piece_to_move,
                &PlacedPiece::drop_by_one,
                &RuleSet::above_bottom,
            ),
        }
    }
}

#[cfg(test)]
mod should {
    use pieces::shape::I;
    use pieces::Position;
    use pieces::*;
    use super::*;

    #[test]
    fn move_current_to_the_left_when_issued_a_move_left_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);

        let piece_keeper = DefaultMotionController::new(RuleSet::default());

        let piece = piece_keeper.move_piece(Command::MoveLeft, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X - MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_the_right_when_issued_a_move_right_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let ruleset = RuleSet::default();
        let piece_keeper = DefaultMotionController::new(ruleset);

        let piece = piece_keeper.move_piece(Command::MoveRight, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X + MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_left_edge_when_issued_a_move_to_left_edge_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let ruleset = RuleSet::default();

        let piece_keeper = DefaultMotionController::new(ruleset);

        let piece = piece_keeper.move_piece(Command::MoveToLeftEdge, piece);

        assert_eq!(
            piece.position,
            Position::new(ruleset.boundary_left, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_right_edge_when_issued_a_move_to_right_edge_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let ruleset = RuleSet::default();
        let piece_keeper = DefaultMotionController::new(ruleset);

        let piece = piece_keeper.move_piece(Command::MoveToRightEdge, piece);

        assert_eq!(
            piece.position,
            Position::new(ruleset.boundary_right, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_bottom_when_issued_a_drop_to_bottom_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let ruleset = RuleSet::default();
        let piece_keeper = DefaultMotionController::new(ruleset);

        let piece = piece_keeper.move_piece(Command::DropToBottom, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X, ruleset.play_area_size.height as i8)
        )
    }
}

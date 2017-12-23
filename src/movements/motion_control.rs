use super::Command;
use super::MotionController;
use super::DefaultMotionController;

use rules::*;
use pieces::PlacedPiece;
use pieces::*;

impl DefaultMotionController {
    fn move_respecting_rule(piece_to_move: PlacedPiece, move_to_apply: &Fn(&PlacedPiece) -> PlacedPiece, rule: &Fn(&PlacedPiece) -> RuleEvaluationResult ) -> PlacedPiece{
        let new_piece = move_to_apply(&piece_to_move);
        match rule(&new_piece) {
            RuleEvaluationResult::Respected => new_piece,
            RuleEvaluationResult::Violated => piece_to_move
        }
    }

    fn keep_moving_while_rule_respected(piece_to_move: PlacedPiece, move_to_apply: &Fn(&PlacedPiece) -> PlacedPiece, rule: &Fn(&PlacedPiece) -> RuleEvaluationResult) -> PlacedPiece {
        let new_piece = move_to_apply(&piece_to_move);
        match rule(&new_piece) {
            RuleEvaluationResult::Respected => DefaultMotionController::keep_moving_while_rule_respected(new_piece, move_to_apply, rule),
            RuleEvaluationResult::Violated => piece_to_move
        }
    }
}

impl MotionController for DefaultMotionController {
    fn move_piece(&self, command: Command, piece_to_move: PlacedPiece) -> PlacedPiece {
        match command {
            Command::MoveLeft => DefaultMotionController::move_respecting_rule(piece_to_move, &PlacedPiece::move_left, &right_of_left_boundary),
            Command::MoveToLeftEdge => DefaultMotionController::keep_moving_while_rule_respected(piece_to_move, &PlacedPiece::move_left, &right_of_left_boundary),
            Command::MoveRight => DefaultMotionController::move_respecting_rule(piece_to_move, &PlacedPiece::move_right, &left_of_right_boundary),
            Command::MoveToRightEdge => DefaultMotionController::keep_moving_while_rule_respected(piece_to_move, &PlacedPiece::move_right, &left_of_right_boundary),
            Command::Drop => DefaultMotionController::move_respecting_rule(piece_to_move, &PlacedPiece::drop_by_one, &above_bottom),
            Command::DropToBottom => DefaultMotionController::keep_moving_while_rule_respected(piece_to_move, &PlacedPiece::drop_by_one, &above_bottom)
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
    fn execute_a_legal_move() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = DefaultMotionController::default();

        let piece = piece_keeper.move_piece(Command::MoveLeft, piece);

        assert_eq!(piece.position.x, ORIGIN_X - 1);
    }

    #[test]
    fn move_current_to_the_left_when_issued_a_move_left_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);

        let piece_keeper = DefaultMotionController::default();

        let piece = piece_keeper.move_piece(Command::MoveLeft, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X - MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_the_right_when_issued_a_move_right_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = DefaultMotionController::default();

        let piece = piece_keeper.move_piece(Command::MoveRight, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X + MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_left_edge_when_issued_a_move_to_left_edge_command() { 
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = DefaultMotionController::default();

        let piece = piece_keeper.move_piece(Command::MoveToLeftEdge, piece);

        assert_eq!(
            piece.position,
            Position::new(BOUNDARY_LEFT, ORIGIN_Y))
    }

    #[test]
    fn move_current_to_right_edge_when_issued_a_move_to_right_edge_command() { 
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = DefaultMotionController::default();

        let piece = piece_keeper.move_piece(Command::MoveToRightEdge, piece);

        assert_eq!(
            piece.position,
            Position::new(BOUNDARY_RIGHT, ORIGIN_Y))
    }

    #[test]
    fn move_current_to_bottom_when_issued_a_drop_to_bottom_command() { 
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = DefaultMotionController::default();

        let piece = piece_keeper.move_piece(Command::DropToBottom, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X, BOUNDARY_BOTTOM))
    }
}
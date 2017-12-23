use super::Command;
use super::MotionController;
use super::DefaultMotionController;

use rules::*;
use pieces::PlacedPiece;
use pieces::*;

impl MotionController for DefaultMotionController {
    fn move_piece(&self, command: Command, piece_to_move: PlacedPiece) -> PlacedPiece {
        match command {
            Command::MoveLeft => {
                let new_piece = piece_to_move.move_left();
                match right_of_left_boundary(&new_piece) {
                    RuleEvaluationResult::Respected => new_piece,
                    RuleEvaluationResult::Violated => piece_to_move
                }
            },
            Command::MoveToLeftEdge => {
                let new_piece = piece_to_move.move_left();
                match right_of_left_boundary(&new_piece) {
                    RuleEvaluationResult::Respected => self.move_piece(Command::MoveToLeftEdge, new_piece.move_left()),
                    RuleEvaluationResult::Violated => piece_to_move
                }
            }
            Command::MoveRight => {
                let new_piece = piece_to_move.move_right();
                match left_of_right_boundary(&new_piece) {
                    RuleEvaluationResult::Respected => new_piece,
                    RuleEvaluationResult::Violated => piece_to_move
                }
            }
            Command::MoveToRightEdge => {
                let new_piece = piece_to_move.move_right();
                match left_of_right_boundary(&new_piece) {
                    RuleEvaluationResult::Respected => self.move_piece(Command::MoveToRightEdge, piece_to_move.move_right()),
                    RuleEvaluationResult::Violated => piece_to_move
                }
            }
            Command::Drop => {
                let new_piece = piece_to_move.drop_by_one();
                match above_bottom(&new_piece) {
                    RuleEvaluationResult::Respected => new_piece,
                    RuleEvaluationResult::Violated => piece_to_move
                }
            }
            Command::DropToBottom => {
                let new_piece = piece_to_move.drop_by_one();
                match above_bottom(&new_piece) {
                    RuleEvaluationResult::Respected => self.move_piece(Command::DropToBottom, piece_to_move.drop_by_one()),
                    RuleEvaluationResult::Violated => piece_to_move
                }
            },
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
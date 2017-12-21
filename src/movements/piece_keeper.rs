use super::Command;
use super::PieceKeeper;
use super::MotionController;
use rules::*;
use pieces::PlacedPiece;

impl PieceKeeper {
    fn check_rules(piece_to_verify: &PlacedPiece) -> bool {
        let mut is_allowed = true;
        is_allowed &= respects_rule(&outside_of_left_boundary, piece_to_verify);
        is_allowed &= respects_rule(&outside_of_right_boundary, piece_to_verify);
        is_allowed &= respects_rule(&outside_of_bottom_boundary, piece_to_verify);
        is_allowed
    }

    fn get_unverified_piece(command: Command, original_piece: PlacedPiece) -> PlacedPiece {
        match command {
            Command::MoveLeft => original_piece.move_left(),
            Command::MoveToLeftEdge => original_piece.move_left(),
            Command::MoveRight => original_piece.move_right(),
            Command::MoveToRightEdge => original_piece.move_right(),
            Command::Drop => original_piece.drop_by_one(),
            Command::DropToBottom => original_piece.drop_by_one(),
            _ => panic!(),
        }
    }
}

impl MotionController for PieceKeeper {
    fn move_piece(&self, command: Command, piece: PlacedPiece) -> PlacedPiece {
        let unverified_piece = PieceKeeper::get_unverified_piece(command, piece);
        let is_allowed = PieceKeeper::check_rules(&unverified_piece);
        if is_allowed {
            match command {
                Command::MoveToLeftEdge => self.move_piece(command, unverified_piece),
                Command::MoveToRightEdge => self.move_piece(command, unverified_piece),
                Command::DropToBottom => self.move_piece(command, unverified_piece),
                _ => unverified_piece
            }
        } else {
            piece
        }
    }
}

fn respects_rule(rule: &Fn(&PlacedPiece) -> RuleEvaluationResult, piece: &PlacedPiece) -> bool {
    match rule(piece) {
        RuleEvaluationResult::Respected => true,
        RuleEvaluationResult::Violated => false,
    }
}

#[cfg(test)]
mod should {
    use game::state::*;
    use pieces::shape::I;
    use pieces::Position;
    use super::*;

    #[test]
    fn execute_a_legal_move() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = PieceKeeper::default();

        let piece = piece_keeper.move_piece(Command::MoveLeft, piece);

        assert_eq!(piece.position.x, ORIGIN_X - 1);
    }

    #[test]
    fn move_current_to_the_left_when_issued_a_move_left_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = PieceKeeper::default();

        let piece = piece_keeper.move_piece(Command::MoveLeft, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X - MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_the_right_when_issued_a_move_right_command() {
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = PieceKeeper::default();

        let piece = piece_keeper.move_piece(Command::MoveRight, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X + MOVE_SPEED, ORIGIN_Y)
        )
    }

    #[test]
    fn move_current_to_left_edge_when_issued_a_move_to_left_edge_command() { 
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = PieceKeeper::default();

        let piece = piece_keeper.move_piece(Command::MoveToLeftEdge, piece);

        assert_eq!(
            piece.position,
            Position::new(BOUNDARY_LEFT, ORIGIN_Y))
    }

    #[test]
    fn move_current_to_right_edge_when_issued_a_move_to_right_edge_command() { 
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = PieceKeeper::default();

        let piece = piece_keeper.move_piece(Command::MoveToRightEdge, piece);

        assert_eq!(
            piece.position,
            Position::new(BOUNDARY_RIGHT, ORIGIN_Y))
    }

    #[test]
    fn move_current_to_bottom_when_issued_a_drop_to_bottom_command() { 
        let piece = PlacedPiece::at_origin_with_shape(I);
        let piece_keeper = PieceKeeper::default();

        let piece = piece_keeper.move_piece(Command::DropToBottom, piece);

        assert_eq!(
            piece.position,
            Position::new(ORIGIN_X, BOUNDARY_BOTTOM))
    }
}
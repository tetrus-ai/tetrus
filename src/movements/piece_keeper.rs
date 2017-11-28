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
        is_allowed
    }

    fn get_unverified_piece(command: Command, original_piece: PlacedPiece) -> PlacedPiece {
        match command {
            Command::MoveLeft => original_piece.move_left(),
            Command::MoveRight => original_piece.move_right(),
            Command::Drop => original_piece.drop_by_one(),
            _ => panic!(),
        }
    }
}

impl MotionController for PieceKeeper {
    fn move_piece(&self, command: Command, piece: PlacedPiece) -> PlacedPiece {
        let unverified_piece = PieceKeeper::get_unverified_piece(command, piece);
        let is_allowed = PieceKeeper::check_rules(&unverified_piece);

        # [allow(unknown_lints)]
        #[allow(match_bool)]
            match is_allowed {
                true => unverified_piece,
                false => piece,
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
}

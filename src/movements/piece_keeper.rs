use super::Command;
use super::PieceKeeper;
use super::super::rules::*;
use super::super::pieces::PlacedPiece;

impl PieceKeeper{
    pub fn execute_command(&self, command: Command, original_piece: PlacedPiece) -> PlacedPiece {
        let unverified_piece = PieceKeeper::get_unverified_piece(command, original_piece);
        let is_allowed = PieceKeeper::check_rules(&unverified_piece);

        match is_allowed {
            true => unverified_piece,
            false => original_piece
        }
    }

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
            Command::Drop => original_piece.drop_by_one()
        }
    }
}

pub fn respects_rule(rule: &Fn(&PlacedPiece) -> RuleEvaluationResult, piece: &PlacedPiece) -> bool {
    match rule(piece){
        RuleEvaluationResult::Respected => true,
        RuleEvaluationResult::Violated => false
    }
}

#[cfg(test)]
mod should{
    use ::game::*;
    use ::movements::Command::*;
    use ::pieces::Shape;
    use super::*;

    #[test]
    fn execute_a_legal_move(){
        let piece = PlacedPiece::at_origin_with_shape(Shape::i());

        let piece_keeper = PieceKeeper::default();

        let piece = piece_keeper.execute_command(MoveLeft, piece);

        assert_eq!(piece.position.x, ORIGIN_X - 1);
    }
}
use engine::piece::PlacedPiece;
use game::{BOUNDARY_LEFT, BOUNDARY_RIGHT};

#[derive(Debug, PartialEq)]
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
    use ::engine::piece::PlacedPiece;
    use ::engine::position::Position;
    use ::objects::shape::Shape;
    use ::objects::shape;
    use ::game::*;
    use super::*;

    const SOME_PIECE: PlacedPiece =
        PlacedPiece::at_origin_with_shape(shape::I);

    #[test]
    fn allow_piece_to_move_to_boundary(){
        let piece = move_to_left_boundary(SOME_PIECE);

        assert_eq!(RuleEvaluationResult::Respected, outside_of_left_boundary(&piece));
    }

    #[test]
    fn not_allow_piece_to_move_beyond_boundary(){
        let piece = move_to_left_boundary(SOME_PIECE);

        let piece = piece.move_left();

        assert_eq!(RuleEvaluationResult::Violated, outside_of_left_boundary(&piece));
    }

    fn move_to_left_boundary(piece: PlacedPiece) -> PlacedPiece{
        match piece.position.x{
            BOUNDARY_LEFT => piece,
            _ => move_to_left_boundary(piece.move_left())
        }
    }
}
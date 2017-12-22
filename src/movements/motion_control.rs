use super::Command;
use super::MotionController;
use super::DefaultMotionController;

use rules::*;
use pieces::PlacedPiece;
use pieces::*;

fn at_left_edge(piece: PlacedPiece) -> bool {
    piece.position.x == BOUNDARY_LEFT
}

fn at_right_edge(piece: PlacedPiece) -> bool {
    piece.position.x == BOUNDARY_RIGHT
}

fn at_bottom(piece: PlacedPiece) -> bool {
    piece.position.y == BOUNDARY_BOTTOM
}

impl MotionController for DefaultMotionController {
    fn move_piece(&self, command: Command, piece_to_move: PlacedPiece) -> PlacedPiece {
        if at_bottom(piece_to_move){ piece_to_move }
        else {
            match command {
                Command::MoveLeft => piece_to_move.move_left(&right_of_left_boundary),
                Command::MoveToLeftEdge => match at_left_edge(piece_to_move) {
                    false => self.move_piece(Command::MoveToLeftEdge, piece_to_move.move_left(&right_of_left_boundary)),
                    true => piece_to_move
                }
                Command::MoveRight => piece_to_move.move_right(&left_of_right_boundary),
                Command::MoveToRightEdge => match at_right_edge(piece_to_move) {
                    false => self.move_piece(Command::MoveToRightEdge, piece_to_move.move_right(&left_of_right_boundary)),
                    true => piece_to_move
                }
                Command::Drop => piece_to_move.drop_by_one(&above_bottom),
                Command::DropToBottom => self.move_piece(Command::DropToBottom, piece_to_move.drop_by_one(&above_bottom)),
            }
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
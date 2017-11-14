use ::engine::piece::Piece;
use ::game::{BOUNDARY_LEFT, BOUNDARY_RIGHT};

pub fn outside_of_left_boundary(&piece: &Piece) -> bool {
    let (x,_) = piece.position.into();
    x < BOUNDARY_LEFT
}

pub fn outside_of_right_boundary(&piece: &Piece) -> bool {
    let (x,_) = piece.position.into();
    x > BOUNDARY_RIGHT
}
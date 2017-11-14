use super::super::piece::Piece;

pub fn outside_of_left_boundary(&piece: &Piece) -> bool {
    let (x,y) = piece.position.into();
    x <= 0
}
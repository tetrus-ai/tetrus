use super::PlacedPiece;
use super::Shape;
use ::game::{ORIGIN, MOVE_SPEED};

impl PlacedPiece {
    pub const fn at_origin_with_shape(shape: Shape) -> Self{
        PlacedPiece {
            shape,
            position: ORIGIN
        }
    }

    pub fn drop_by_one(&self) -> Self{
        let position = self.position.add_to_y(MOVE_SPEED);
        PlacedPiece {
            shape: self.shape,
            position
        }
    }

    pub fn move_left(&self) -> Self{
        PlacedPiece {
            shape: self.shape,
            position: self.position.subtract_from_x(MOVE_SPEED)
        }
    }

    pub fn move_right(&self) -> Self{
        let position = self.position.add_to_x(MOVE_SPEED);
        PlacedPiece {
            shape: self.shape,
            position
        }
    }
}

#[cfg(test)]
mod should{
    use super::PlacedPiece;
    use super::super::placed_piece;
    use ::game::MOVE_SPEED;
    use ::pieces::Shape;

    #[test]
    fn decrease_x_by_one_when_moved_left(){
        let current = PlacedPiece::at_origin_with_shape(Shape::i());
        let initial_x = current.position.x;
        let initial_y = current.position.y;
        let expected_x = initial_x - MOVE_SPEED;
        let current = current.move_left();

        assert_eq!(current.position.x, expected_x);
        assert_eq!(current.position.y, initial_y);
    }

    #[test]
    fn increase_x_by_one_when_moved_right(){
        let current = PlacedPiece::at_origin_with_shape(Shape::i());
        let initial_x = current.position.x;
        let initial_y = current.position.y;
        let expected_x = initial_x + MOVE_SPEED;
        let current = current.move_right();

        assert_eq!(current.position.x, expected_x);
        assert_eq!(current.position.y, initial_y);
    }
}
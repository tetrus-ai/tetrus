use ::objects::shape::Shape;
use super::position::Position;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub shape: Shape,
    pub position: Position
}

impl Piece {
    pub fn new(shape: Shape) -> Self{
        Piece {
            shape,
            position: Position::new(0, 0)
        }
    }

    pub fn drop_by_one(&self) -> Self{
        let position = self.position.add_to_y(1);
        Piece {
            shape: self.shape,
            position
        }
    }

    pub fn move_left(&self) -> Self{
        Piece{
            shape: self.shape,
            position: self.position.subtract_from_x(1)
        }
    }

    pub fn move_right(&self) -> Self{
        let position = self.position.add_to_x(1);
        Piece {
            shape: self.shape,
            position
        }
    }
}

#[cfg(test)]
mod should{
    use super::Piece;
    use ::objects::shape::Shape;

    #[test]
    fn decrease_x_by_one_when_moved_left(){
        let current = Piece::new(Shape::i());
        let initial_x = current.position.x;
        let initial_y = current.position.y;
        let expected_x = initial_x - 1;
        let current = current.move_left();

        assert_eq!(current.position.x, expected_x);
        assert_eq!(current.position.y, initial_y);
    }

    #[test]
    fn increase_x_by_one_when_moved_right(){
        let current = Piece::new(Shape::i());
        let initial_x = current.position.x;
        let initial_y = current.position.y;
        let expected_x = initial_x + 1;
        let current = current.move_right();

        assert_eq!(current.position.x, expected_x);
        assert_eq!(current.position.y, initial_y);
    }
}
use ::objects::tetromino::Tetromino;
use super::position::Position;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Current{
    pub tetromino: Tetromino,
    pub position: Position
}

impl Current{
    pub fn new(tetromino: Tetromino) -> Self{
        Current{
            tetromino,
            position: Position::new(5, 2)
        }
    }

    pub fn drop_by_one(&self) -> Current{
        let position = self.position.add_to_y(1);
        Current{
            tetromino: self.tetromino,
            position
        }
    }

    pub fn move_left(&self) -> Option<Current>{
        match self.position.subtract_from_x(1) {
            None => None,
            Some(x) => Some(Current{
                tetromino: self.tetromino,
                position: x
            })
        }
    }

    pub fn move_right(&self) -> Current{
        let position = self.position.add_to_x(1);
        Current{
            tetromino: self.tetromino,
            position
        }
    }
}

#[cfg(test)]
mod should{
    use super::Current;
    use ::engine::position::Position;
    use ::objects::tetromino::Tetromino;

    #[test]
    fn decrease_x_by_one_when_moved_left(){
        let current = Current::new(Tetromino::i());
        let initial_x = current.position.x;
        let initial_y = current.position.y;
        let expected_x = initial_x - 1;
        let current = current.move_left().unwrap();

        assert_eq!(current.position.x, expected_x);
        assert_eq!(current.position.y, initial_y);
    }

    #[test]
    fn increase_x_by_one_when_moved_right(){
        let current = Current::new(Tetromino::i());
        let initial_x = current.position.x;
        let initial_y = current.position.y;
        let expected_x = initial_x + 1;
        let current = current.move_right();

        assert_eq!(current.position.x, expected_x);
        assert_eq!(current.position.y, initial_y);
    }

    #[test]
    fn return_none_if_asked_to_move_to_negative_numbers(){
        let mut current = Current::new(Tetromino::i());
        current.position = Position::new(0,0);

        let current = current.move_left();

        assert_eq!(current, None);
    }
}
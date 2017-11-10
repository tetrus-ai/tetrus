#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        Position { x, y }
    }

    pub fn add_to_y(&self, delta_y: i8) -> Position {
        Position {
            x: self.x,
            y: self.y + delta_y,
        }
    }

    pub fn subtract_from_x(&self, delta_x: i8) -> Position {
        Position {
            x: self.x - delta_x,
            y: self.y,
        }
    }

    pub fn add_to_x(&self, delta_x: i8) -> Position {
        Position {
            x: self.x + delta_x,
            y: self.y,
        }
    }
}

#[cfg(test)]
mod should {
    use super::Position;

    #[test]
    fn only_add_to_x_when_asked_to_add_to_x() {
        let initial_x = 1;
        let initial_y = 2;
        let expected_x = 7;
        let position = Position::new(initial_x, initial_y);

        let position = position.add_to_x(6);

        assert_eq!(position.x, expected_x);
        assert_eq!(position.y, initial_y);
    }

    #[test]
    fn only_subtract_from_x_when_asked_to_subtract_from_x() {
        let initial_x = 6;
        let initial_y = 2;
        let expected_x = 3;
        let position = Position::new(initial_x, initial_y);

        let position = position.subtract_from_x(3);

        assert_eq!(position.x, expected_x);
        assert_eq!(position.y, initial_y);
    }

    #[test]
    fn only_add_to_y_when_asked_to_add_to_y() {
        let initial_x = 1;
        let initial_y = 2;
        let expected_y = 7;
        let position = Position::new(initial_x, initial_y);

        let position = position.add_to_y(5);

        assert_eq!(position.x, initial_x);
        assert_eq!(position.y, expected_y);
    }

    #[test]
    fn allow_negative_positions() {
        let initial_x = 2;
        let initial_y = 2;
        let expected_x = -1;

        let position = Position::new(initial_x, initial_y);

        let position = position.subtract_from_x(3);

        assert_eq!(position.x, expected_x);
        assert_eq!(position.y, initial_y);
        
    }
}
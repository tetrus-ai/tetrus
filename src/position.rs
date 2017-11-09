#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position{
    x: u8,
    y: u8
}

impl Position{
    pub fn new(x: u8, y: u8) -> Self{
        Position{
            x,
            y
        }
    }

    pub fn add_to_y(&self, delta_y: u8) -> Position{
        Position{
            x: self.x,
            y: self.y + delta_y
        }
    }
}
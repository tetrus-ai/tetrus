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
}
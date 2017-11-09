#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tetromino {
    Invalid,
    L,
    J,
    T,
    S,
    Z,
    O,
    I
}

impl Default for Tetromino {
    fn default() -> Self {
        Tetromino::Invalid
    }
}
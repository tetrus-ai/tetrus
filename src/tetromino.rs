use ::rand::distributions::range::SampleRange;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tetromino {
    I,
    J,
    L,
    S,
    T,
    O,
    Z,
}
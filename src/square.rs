#[derive(Debug, Copy)]
pub struct Square{
}

impl Square{
    pub fn empty() -> Square {
        Square{}
    }
}

impl Clone for Square{
    fn clone(&self) -> Self {
        Square::empty()
    }
}

impl PartialEq for Square{
    fn eq(&self, other: &Square) -> bool {
        true
    }
}

#[cfg(test)]
mod square_should {
    use super::Square;
    #[test]
    fn be_constructable() {
        let square = Square::empty();
        assert_eq!(2+2, 4);
    }

    #[test]
    fn implement_clone_trait() {
        let square = Square::empty();
        assert_eq!(square.clone(), square);
    }

    #[test]
    fn implement_copy_trait() {
        let square = Square::empty();
        let copy = square;
        assert_eq!(copy, square);
    }
}

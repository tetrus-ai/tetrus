use super::square::Square;
#[derive(Clone, Copy, Debug, Default)]
pub struct Well{
    // should be private
    pub items: [[Square; 10]; 20]
}

impl Well{
    pub fn empty() -> Self
    {
        Well{
            items: [[Square::empty(); 10]; 20]
        }
    }
}

impl PartialEq for Well{
    fn eq(&self, _other: &Well) -> bool {
        true
    }
}

#[cfg(test)]
mod empty_well_should {
    use super::Well;
    use ::square::Square;

    #[test]
    fn be_empty() {
        let well = Well::empty();
        let empty_square = &Square::empty();
        for (_i, row) in well.items.iter().enumerate() {
            for(_j, well_square) in row.iter().enumerate() {
                assert_eq!(well_square, empty_square);
            }
        }
    }
}
use super::square::Square;

#[derive(Debug)]
pub struct Well{
    // should be private
    pub items: [[Square; 10]; 20]
}

impl Well{
    pub fn empty() -> Well
    {
        Well{
            items: [[Square::empty(); 10]; 20]
        }
    }
}

impl PartialEq for Well{
    fn eq(&self, other: &Well) -> bool {
        true
    }
}

#[cfg(test)]
mod well_should {
    use super::Well;
    #[test]
    fn be_equatable() {
        assert_eq!(Well::empty(), Well::empty())
    }
}

#[cfg(test)]
mod empty_well_should {
    use super::Well;
    use ::square::Square;

    #[test]
    fn be_empty() {
        let well = Well::empty();
        for (i, row) in well.items.iter().enumerate() {
            for(j, well_square) in row.iter().enumerate() {
                assert_eq!(well_square, &Square::empty());
            }
        }
    }
}
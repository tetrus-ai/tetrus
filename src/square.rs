#[derive(Debug, Default, Copy, Clone)]
pub struct Square{
}

impl Square{
    pub fn empty() -> Square {
        Square{}
    }
}

impl PartialEq for Square{
    fn eq(&self, _other: &Square) -> bool {
        true
    }
}
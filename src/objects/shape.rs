#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {
}

pub const I: Shape = Shape{};

impl Shape {
    fn new() -> Self{
        Shape {
        }
    }

    pub fn i() -> Self{
        Shape::new()
    }
    pub fn j() -> Self{
        Shape::new()
    }
    pub fn l() -> Self{
        Shape::new()
    }
    pub fn s() -> Self{
        Shape::new()
    }
    pub fn t() -> Self{
        Shape::new()
    }
    pub fn o() -> Self{
        Shape::new()
    }
    pub fn z() -> Self{
        Shape::new()
    }
}
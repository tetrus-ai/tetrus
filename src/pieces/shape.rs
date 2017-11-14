use super::Shape;

pub const I: Shape = Shape{};

impl Shape {
    const fn new() -> Self{
        Shape {
        }
    }

    pub const fn i() -> Self{
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
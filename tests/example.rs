use ops_derive2::*;

#[derive(Debug, PartialEq, AutoAdd)]
pub struct Point(u32, u32, u32);

impl Point {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self(x, y, z)
    }
}

#[test]
fn main() {
    let a = Point::new(1, 2, 3);
    let b = Point::new(4, 5, 6);
    let c = Point::new(5, 7, 9);

    assert_eq!(a + b, c);
}

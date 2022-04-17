extern crate num;
use num::FromPrimitive;

pub struct Triangle<T>(T, T, T);

impl<T: std::ops::Add<Output = T> + std::cmp::PartialOrd + Copy + FromPrimitive> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides[0] + sides[1] < sides[2]
            || sides[0] + sides[2] < sides[1]
            || sides[1] + sides[2] < sides[0]
            || sides[0] == FromPrimitive::from_u64(0).expect("Can't convert 0 to the type of input")
            || sides[1] == FromPrimitive::from_u64(0).expect("Can't convert 0 to the type of input")
            || sides[2] == FromPrimitive::from_u64(0).expect("Can't convert 0 to the type of input")
        {
            return None;
        }
        Some(Triangle(sides[0], sides[1], sides[2]))
    }

    pub fn is_equilateral(&self) -> bool {
        if self.0 == self.1 && self.0 == self.2 && self.1 == self.2 {
            return true;
        }
        false
    }

    pub fn is_scalene(&self) -> bool {
        if self.0 != self.1 && self.0 != self.2 && self.1 != self.2 {
            return true;
        }
        false
    }

    pub fn is_isosceles(&self) -> bool {
        if self.0 == self.1 || self.0 == self.2 || self.1 == self.2 {
            return true;
        }
        false
    }
}

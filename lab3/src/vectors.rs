use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

struct Vec2D {
    x : f32,
    y: f32
}

impl Vec2D {
    fn new_unit_vec() -> Vec2D {
        Vec2D{x: 1.0, y: 1.0}
    }
    
    fn equal(&self, other : Vec2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector 2d: x = {}, y = {}", self.x, self.y)
    }
}

impl Add for Vec2D {
    type Output = Self;
 
    fn add(self, other: Vec2D) -> Self {
        Vec2D{x: self.x + other.x, y: self.y + other.y}
    }
}
 
impl Sub for Vec2D {
    type Output = Self;
 
    fn sub(self, other: Vec2D) -> Self {
        Vec2D{x: self.x - other.x, y: self.y - other.y}
    }
}


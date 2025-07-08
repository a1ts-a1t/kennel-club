use std::ops::Sub;

use serde::Serialize;

#[derive(Serialize, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn squared_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn dot(vec1: &Self, vec2: &Self) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

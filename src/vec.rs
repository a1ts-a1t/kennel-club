use std::ops::{Add, Sub};

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
}

impl Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}


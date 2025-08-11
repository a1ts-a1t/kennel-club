use std::ops::Sub;

#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Vec2 {
    fn from(value: (f64, f64)) -> Self {
        Vec2 {
            x: value.0,
            y: value.1,
        }
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

impl Vec2 {
    pub fn squared_norm(&self) -> f64 {
        self.x * self.x + self.y + self.y
    }
}

pub type Position = Vec2;
pub type Delta = Vec2;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn squared_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}


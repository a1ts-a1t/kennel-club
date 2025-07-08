use serde::Serialize;
use std::cmp::Ordering;
use std::ops::Sub;

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
pub enum RealQuadraticRoots {
    Double(f64, f64),
    Single(f64),
    None,
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> RealQuadraticRoots {
    let discriminant = b * b - 4f64 * a * c;
    match discriminant.partial_cmp(&0f64).unwrap() {
        Ordering::Less => RealQuadraticRoots::None,
        Ordering::Equal => RealQuadraticRoots::Single(-b / (2f64 * a)),
        Ordering::Greater => {
            let sqrt_discriminant = discriminant.sqrt();
            let root1 = (-b + sqrt_discriminant) / (2f64 * a);
            let root2 = (-b - sqrt_discriminant) / (2f64 * a);
            RealQuadraticRoots::Double(root1, root2)
        }
    }
}

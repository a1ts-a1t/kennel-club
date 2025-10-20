use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Neg, Sub},
};

use rand::Rng;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
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

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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

impl Sub<&f64> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &f64) -> Self::Output {
        &self - &Vec2::new(*rhs, *rhs)
    }
}

impl Mul<&Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<&Vec2> for &f64 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<f64> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn squared_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn norm(&self) -> f64 {
        self.squared_norm().sqrt()
    }

    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn normalized(&self) -> Self {
        let magnitude = self.squared_norm().sqrt();
        self / magnitude
    }

    pub fn with_norm(&self, norm: f64) -> Self {
        let scale = self.squared_norm().sqrt() * norm;
        scale * self
    }

    pub fn dot(v1: &Self, v2: &Self) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let theta = rng.random_range(0.0..(2.0 * PI));
        (theta.cos(), theta.sin()).into()
    }
}

use crate::math::Vec2;

/**
 * A spherical collidable object
 */
#[derive(Clone, Debug)]
pub struct Collidable {
    pub position: Vec2,
    pub radius: f64,
}

impl Collidable {
    pub fn new(position: Vec2, radius: f64) -> Self {
        Collidable { position, radius }
    }

    /**
     * Returns if two collidables are currently colliding
     * ie in each other's collision boxes.
     */
    pub fn is_colliding(&self, other: &Collidable) -> bool {
        let delta = &self.position - &other.position;
        let threshold = self.radius + other.radius;
        delta.squared_norm() < (threshold * threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_colliding() {
        let collidable = Collidable::new((1.0, 1.0).into(), 1.0);

        let colliding = Collidable::new((1.5, 1.5).into(), 1.0);
        let not_colliding = Collidable::new((3.0, 1.0).into(), 1.0);

        assert!(collidable.is_colliding(&colliding));
        assert!(!collidable.is_colliding(&not_colliding));
    }
}

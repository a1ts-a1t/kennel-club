use crate::math::{Vec2, approx};

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
     * Returns if two collidables are currently colliding.
     *
     * Note: Bordering (or close to bordering) does not count as colliding)
     */
    pub fn is_colliding(&self, other: &Collidable) -> bool {
        let delta = &self.position - &other.position;
        let threshold = self.radius + other.radius;

        let delta2 = delta.squared_norm();
        let threshold2 = threshold * threshold;

        approx::lt(&delta2, &threshold2)
    }

    pub fn is_out_of_unit_bounds(&self) -> bool {
        // let lower_bound = 0.0 + self.radius;
        if approx::lt(&self.position.x, &self.radius) || approx::lt(&self.position.y, &self.radius)
        {
            return true;
        }

        let upper_bound = 1.0 - self.radius;
        if approx::gt(&self.position.x, &upper_bound) || approx::gt(&self.position.y, &upper_bound)
        {
            return true;
        }

        false
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
        let bordering = Collidable::new((1.0, 3.0).into(), 1.0);

        assert!(collidable.is_colliding(&colliding));
        assert!(!collidable.is_colliding(&not_colliding));
        assert!(!collidable.is_colliding(&bordering));
    }
}

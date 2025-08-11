use crate::vec2::{Delta, Position, Vec2};

/**
 * Represents a spherical collidable object
 */
#[derive(Clone)]
pub struct Collidable {
    pub position: Position,
    pub radius: f64,
}

impl Collidable {
    pub fn new(position: Position, radius: f64) -> Self {
        Collidable { position, radius }
    }

    /**
     * Given an amount to move (delta) in a time step
     * and the equation of a half plane, return the collidable
     * whose position is the current collidable moved
     * as much in the direction of delta as possible without
     * going beyond delta while still remaining in the halfplane.
     *
     * That is, let p(t) = self.position + t * delta.
     * This function returns p(t')
     * where t' is the maximal value of the set whose members, t, satisfy
     * 1. t in [0, 1]
     * 2. p(t).x * coeffs.x + p(t).y * coeffs.y <= offset
     *
     * Return err if the current position is outside the half plane.
     */
    pub fn resolve_to_halfplane(
        &self,
        delta: Delta,
        coeffs: Vec2,
        offset: f64,
    ) -> Result<Collidable, ()> {
        todo!();
    }

    /**
     * Given an amount to move (delta) in a timestep
     * and another collidable, return the collidable whose position
     * is the current collidable moved as much in the direction
     * of delta as possible without going beyond delta
     * while having both collidables not in each other's collision boxes.
     *
     * That is, let p(t) = self.position + t * delta.
     * This function returns p(t')
     * where t' is the maximal value of the set whose members, t, satisfy
     * 1. t in [0, 1]
     * 2. for all p in { p : norm(p(t) - p) <= self.radius },
     *    norm(other.position - p) >= other.radius
     *
     * Return err if the current is already colliding with other
     * (ie there exists a p that satisfies
     * 1. norm(self.position - p) <= self.radius
     * 2. norm(other.position - p) <= other.radius
     */
    pub fn resolve_to_collidable(
        &self,
        delta: Delta,
        other: &Collidable,
    ) -> Result<Collidable, ()> {
        todo!();
    }

    /**
     * Returns if two collidables are currently colliding
     * ie in each other's collision boxes.
     */
    pub fn is_colliding(&self, other: &Collidable) -> bool {
        let delta = &self.position - &other.position;
        let threshold = self.radius + other.radius;
        delta.squared_norm() < threshold * threshold
    }
}

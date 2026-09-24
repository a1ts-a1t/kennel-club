use crate::{math::Vec2, physics::collider::Collider};

#[derive(Clone, Copy, Debug)]
pub struct Body {
    collider: Collider,
    velocity: Vec2,
    inverse_mass: f64,
}

/// given an array of bodies, take one time step
/// with them and return the resultant, decolided bodies
/// in the same order as was given
pub fn resolve_bodies(bodies: &[Body]) -> Vec<Vec2> {
    todo!()
}

use crate::{
    math::Vec2,
    physics::collider::{Collider, normal, signed_distance},
};

const DISTANCE_TOLERANCE: f64 = 1e-12;

#[derive(Clone, Copy, Debug)]
pub struct Body {
    collider: Collider,
    velocity: Vec2,
    inverse_mass: f64,
}

impl Body {
    fn mut_translate(&mut self, p: Vec2) {
        self.collider = self.collider.translate(p);
    }

    fn mut_impel(&mut self, v: Vec2) {
        self.velocity = self.velocity + v;
    }

    fn step(&self, t: f64) -> Self {
        let delta = t * self.velocity;

        Self {
            collider: self.collider.translate(delta),
            velocity: self.velocity,
            inverse_mass: self.inverse_mass,
        }
    }
}

/// given an array of bodies, take one time step
/// with them and return the resultant, decolided bodies
/// in the same order as was given
pub fn resolve_bodies(_bodies: &[Body]) -> Vec<Vec2> {
    // TODO: number of substeps can be dynamically solved for
    let _k = 64; // number of substeps
    todo!()
}

fn resolve_all(_bodies: &mut [Body]) -> Result<bool, ()> {
    todo!()
}

/// given two bodies, mutate them such that they are not colliding
/// return Ok(true) if any mutations happened and Ok(false) if no mutations happened
/// errors when two bodies colliders cannot be uncollided (ie two half planes)
fn resolve(a: &mut Body, b: &mut Body) -> Result<bool, ()> {
    let d = signed_distance(a.collider, b.collider).ok_or(())?;

    // the bodies are neither inside each other or touching
    if d > DISTANCE_TOLERANCE {
        return Ok(false);
    }

    let (inverse_mass_a, inverse_mass_b) = (a.inverse_mass, b.inverse_mass);
    let total_inverse_mass = inverse_mass_a + inverse_mass_b;

    // TODO: figure out what to do when colliders don't have a normal
    // like two non-antiparallel half planes
    // or don't have a signed distance
    let normal = normal(a.collider, b.collider).ok_or(())?;

    // we need to reposition something but both bodies have infinite mass
    if d < DISTANCE_TOLERANCE && total_inverse_mass == 0f64 {
        return Err(());
    }

    let (weight_a, weight_b) = (
        inverse_mass_a / total_inverse_mass,
        inverse_mass_b / total_inverse_mass,
    );

    // push the two bodies to be at least DISTANCE_TOLERANCE apart
    if d < DISTANCE_TOLERANCE {
        let deficit = DISTANCE_TOLERANCE - d;

        // translate both bodies oppositely against the contact normal
        // proportionally to their inverse masses
        // such that the sum of how much they move is equal to the deficit
        a.mut_translate(deficit * weight_a * normal);
        b.mut_translate(-deficit * weight_b * normal);
    }

    // the bodies also need their velocities adjusted
    // or else they will keep running into each other

    let approach_factor = Vec2::dot(a.velocity - b.velocity, normal);
    if approach_factor < 0f64 {
        a.mut_impel(-approach_factor * weight_a * normal);
        b.mut_impel(approach_factor * weight_b * normal);
    }

    // return true if a/b got mutated
    Ok(d < DISTANCE_TOLERANCE || approach_factor < 0f64)
}

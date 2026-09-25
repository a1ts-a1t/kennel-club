use crate::{
    math::Vec2,
    physics::collider::{Collider, normal, signed_distance},
};

const DISTANCE_TOLERANCE: f64 = 1e-12;
const MAX_PASSES: usize = 64;

#[derive(Clone, Debug)]
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

    fn mut_step(&mut self, t: f64) {
        let delta = t * self.velocity;
        self.collider = self.collider.translate(delta);
    }
}

/// given an array of bodies, take one time step
/// with them and return the resultant, decolided bodies
/// in the same order as was given
/// resultant bodies will have stepped positions and the same inverse mass
pub fn step(bodies: &[Body]) -> Result<Vec<Body>, ()> {
    // TODO: number of substeps can be dynamically solved for
    let k = 64; // number of substeps

    let mut substeps: Vec<Body> = bodies.iter().cloned().collect();

    for _ in 0..k {
        for substep in substeps.iter_mut() {
            substep.mut_step(1f64 / k as f64);
        }

        for _ in 0..(MAX_PASSES - 1) {
            if !resolve_all_positions(&mut substeps)? {
                break;
            }
        }

        if resolve_all_positions(&mut substeps)? {
            return Err(());
        }

        resolve_all_velocities(&mut substeps)?;
    }

    for _ in 0..(MAX_PASSES - 1) {
        if !resolve_all_positions(&mut substeps)? {
            break;
        }
    }

    if resolve_all_positions(&mut substeps)? {
        return Err(());
    }

    resolve_all_velocities(&mut substeps)?;

    Ok(substeps)
}

/// given a bunch of bodies, mutate their positions such that they are not colliding
fn resolve_all_positions(bodies: &mut [Body]) -> Result<bool, ()> {
    let mut mutated = false;

    let mut rest = bodies;
    while let Some((first, tail)) = rest.split_first_mut() {
        for other in tail.iter_mut() {
            mutated |= resolve_positions(first, other)?;
        }

        rest = tail;
    }

    Ok(mutated)
}

/// given a bunch of bodies, mutate their velocities so they don't run into each other
fn resolve_all_velocities(bodies: &mut [Body]) -> Result<(), ()> {
    let mut rest = bodies;
    while let Some((first, tail)) = rest.split_first_mut() {
        for other in tail.iter_mut() {
            resolve_velocities(first, other)?;
        }

        rest = tail;
    }

    Ok(())
}

/// given two bodies, mutate them such that they are not colliding
/// return if the position mutated
/// errors when two bodies colliders cannot be uncollided (ie two half planes)
fn resolve_positions(a: &mut Body, b: &mut Body) -> Result<bool, ()> {
    let d = signed_distance(&a.collider, &b.collider).ok_or(())?;

    // the bodies are neither inside each other or touching
    if d >= DISTANCE_TOLERANCE {
        return Ok(false);
    }

    let (inverse_mass_a, inverse_mass_b) = (a.inverse_mass, b.inverse_mass);
    let total_inverse_mass = inverse_mass_a + inverse_mass_b;

    // TODO: figure out what to do when colliders don't have a normal
    // like two non-antiparallel half planes
    // or don't have a signed distance
    let normal = normal(&a.collider, &b.collider).ok_or(())?;

    // we need to reposition something but both bodies have infinite mass
    if total_inverse_mass == 0f64 {
        return Err(());
    }

    let (weight_a, weight_b) = (
        inverse_mass_a / total_inverse_mass,
        inverse_mass_b / total_inverse_mass,
    );

    // push the two bodies to be at least DISTANCE_TOLERANCE apart
    let deficit = DISTANCE_TOLERANCE - d;

    // translate both bodies oppositely against the contact normal
    // proportionally to their inverse masses
    // such that the sum of how much they move is equal to the deficit
    a.mut_translate(deficit * weight_a * normal);
    b.mut_translate(-deficit * weight_b * normal);

    Ok(true)
}

fn resolve_velocities(a: &mut Body, b: &mut Body) -> Result<(), ()> {
    let d = signed_distance(&a.collider, &b.collider).ok_or(())?;

    // the bodies aren't touching
    if d > DISTANCE_TOLERANCE {
        return Ok(());
    }

    let (inverse_mass_a, inverse_mass_b) = (a.inverse_mass, b.inverse_mass);
    let total_inverse_mass = inverse_mass_a + inverse_mass_b;

    // TODO: figure out what to do when colliders don't have a normal
    let normal = normal(&a.collider, &b.collider).ok_or(())?;

    // we need to impel something but both bodies have infinite mass
    if total_inverse_mass == 0f64 {
        return Err(());
    }

    let (weight_a, weight_b) = (
        inverse_mass_a / total_inverse_mass,
        inverse_mass_b / total_inverse_mass,
    );

    let approach_factor = Vec2::dot(a.velocity - b.velocity, normal);
    if approach_factor < 0f64 {
        a.mut_impel(-approach_factor * weight_a * normal);
        b.mut_impel(approach_factor * weight_b * normal);
    };

    Ok(())
}

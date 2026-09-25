use crate::{
    math::Vec2,
    physics::{Collider, normal, signed_distance},
};

const DISTANCE_TOLERANCE: f64 = 1e-12;
const LINEAR_SLOP: f64 = 1e-15;
const MAX_PASSES: usize = 256;

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

    pub fn new(collider: Collider) -> Self {
        Body {
            collider,
            velocity: Vec2::zero(),
            inverse_mass: 0f64,
        }
    }

    pub fn with_velocity(self, v: Vec2) -> Self {
        Body {
            collider: self.collider,
            velocity: v,
            inverse_mass: self.inverse_mass,
        }
    }

    pub fn with_mass(self, m: f64) -> Self {
        Body {
            collider: self.collider,
            velocity: self.velocity,
            inverse_mass: 1f64 / m,
        }
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn collider(&self) -> Collider {
        self.collider.clone()
    }
}

/// given an array of bodies, take one time step
/// with them and return the resultant, decolided bodies
/// in the same order as was given
/// resultant bodies will have stepped positions and the same inverse mass
pub fn step(bodies: &[Body], k: Option<usize>) -> Result<Vec<Body>, ()> {
    let k = k.unwrap_or(8);

    let mut substeps: Vec<Body> = bodies.to_vec();

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
    if d > LINEAR_SLOP {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn circle(x: f64, y: f64, radius: f64, velocity: Vec2) -> Body {
        Body::new(Collider::new_circle(Vec2::new(x, y), radius))
            .with_velocity(velocity)
            .with_mass(radius)
    }

    fn wall() -> Body {
        let collider = Collider::new_plane(Vec2::new(-1.0, 0.0), 0.0)
            .union(Collider::new_plane(Vec2::new(1.0, 0.0), 1.0))
            .union(Collider::new_plane(Vec2::new(0.0, -1.0), 0.0))
            .union(Collider::new_plane(Vec2::new(0.0, 1.0), 1.0));
        Body::new(collider)
            .with_velocity(Vec2::zero())
            .with_mass(f64::INFINITY)
    }

    fn center(body: &Body) -> Vec2 {
        match &body.collider {
            Collider::Circle { center, .. } => *center,
            _ => panic!("expected a circle body"),
        }
    }

    fn assert_resolved(bodies: &[Body]) {
        for (i, a) in bodies.iter().enumerate() {
            let (p, r) = match &a.collider {
                Collider::Circle { center, radius } => (*center, *radius),
                _ => continue,
            };
            assert!(
                p.x >= r && p.x <= 1.0 - r && p.y >= r && p.y <= 1.0 - r,
                "body {i} escaped the kennel at ({}, {})",
                p.x,
                p.y
            );

            for (j, b) in bodies.iter().enumerate().skip(i + 1) {
                let gap = signed_distance(&a.collider, &b.collider)
                    .expect("gap must exist for circle bodies");
                assert!(
                    gap > 0.0,
                    "bodies {i} and {j} ended in contact (gap {gap:e})"
                );
            }
        }
    }

    #[test]
    fn resting_pair_settles_without_error() {
        let bodies = step(
            &[
                circle(0.45, 0.5, 0.05, Vec2::zero()),
                circle(0.55 - 1e-9, 0.5, 0.05, Vec2::zero()),
                wall(),
            ],
            None,
        )
        .expect("resting pair must resolve");
        assert_resolved(&bodies);
    }

    #[test]
    fn pressure_chain_converges() {
        let bodies = step(
            &[
                circle(0.050000000001, 0.949999999999, 0.05, Vec2::zero()),
                circle(0.156, 0.900, 0.05, Vec2::new(-0.043, 0.025)),
                circle(0.238, 0.842, 0.05, Vec2::new(-0.044, 0.025)),
                wall(),
            ],
            Some(2),
        )
        .expect("pressure chain must converge");
        assert_resolved(&bodies);
    }

    #[test]
    fn head_on_pair_does_not_pass_through() {
        let bodies = step(
            &[
                circle(0.3, 0.5, 0.05, Vec2::new(0.05, 0.0)),
                circle(0.7, 0.5, 0.05, Vec2::new(-0.05, 0.0)),
                wall(),
            ],
            None,
        )
        .expect("head-on pair must resolve");
        assert_resolved(&bodies);
        assert!(
            center(&bodies[0]).x < center(&bodies[1]).x,
            "creatures passed through each other"
        );
    }

    #[test]
    fn fast_pair_does_not_tunnel() {
        let bodies = step(
            &[
                circle(0.45, 0.5, 0.05, Vec2::new(0.15, 0.0)),
                circle(0.55, 0.5, 0.05, Vec2::new(-0.15, 0.0)),
                wall(),
            ],
            Some(6),
        )
        .expect("fast pair must resolve");
        assert_resolved(&bodies);
        assert!(
            center(&bodies[0]).x < center(&bodies[1]).x,
            "creatures tunneled through each other"
        );
    }

    #[test]
    fn wall_stop() {
        let bodies = step(
            &[circle(0.95, 0.5, 0.1, Vec2::new(0.05, 0.0)), wall()],
            None,
        )
        .expect("wall stop must resolve");

        let x = center(&bodies[0]).x;
        assert!(
            (x - 0.9).abs() < 1e-9,
            "creature did not stop at the wall inset: x = {x}"
        );
    }
}

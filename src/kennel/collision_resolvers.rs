use itertools::{Itertools, zip_eq};

use crate::kennel::KennelMetadata;
use crate::math::{RealQuadraticRoots, Vec2, solve_quadratic};

use super::collision_lattice::CollisionLattice;

fn resolve_creature_collision(
    new_position: &Vec2,
    original_position: &Vec2,
    check_position: &Vec2,
    metadata: &KennelMetadata,
) -> Option<Vec2> {
    let delta_new = new_position - original_position;
    let a = delta_new.squared_norm();
    let desired_squared_norm = 4f64 * metadata.creature_radius * metadata.creature_radius;

    if a >= desired_squared_norm {
        return None;
    }

    let delta_check = check_position - original_position;
    let b = 2f64 * Vec2::dot(&delta_check, &delta_new);
    let c = delta_check.squared_norm() - desired_squared_norm;

    let t = match solve_quadratic(a, b, c) {
        RealQuadraticRoots::Double(root, _) if root > 0f64 && root < 1f64 => root,
        RealQuadraticRoots::Double(_, root) if root > 0f64 && root < 1f64 => root,
        RealQuadraticRoots::Single(root) if root > 0f64 && root < 1f64 => root,
        _ => 0f64, // this shouldn't happen, but if it does, default to zero
    };

    Some(Vec2 {
        x: original_position.x + delta_new.x * t,
        y: original_position.y + delta_new.y * t,
    })
}

fn resolve_wall_collision(
    new_position: &Vec2,
    original_position: &Vec2,
    metadata: &KennelMetadata,
) -> Option<Vec2> {
    let mut t = 1f64;

    if new_position.x < metadata.creature_radius {
        let delta_x = new_position.x - original_position.x;
        t = (metadata.creature_radius - original_position.x) / delta_x
    } else if new_position.x > (metadata.width - metadata.creature_radius) {
        let delta_x = new_position.x - original_position.x;
        t = (metadata.width - metadata.creature_radius - original_position.x) / delta_x
    }

    if new_position.y < metadata.creature_radius {
        let delta_y = new_position.y - original_position.y;
        t = f64::min(
            (metadata.creature_radius - original_position.y) / delta_y,
            t,
        );
    } else if new_position.y > (metadata.height - metadata.creature_radius) {
        let delta_y = new_position.y - original_position.y;
        t = f64::min(
            (metadata.height - metadata.creature_radius - original_position.y) / delta_y,
            t,
        );
    }

    // check if there was no collision
    if t >= 1f64 {
        return None;
    }

    // calculate new position
    let delta = new_position - original_position;
    Some(Vec2 {
        x: original_position.x + delta.x * t,
        y: original_position.y + delta.y * t,
    })
}

pub fn resolve_collisions(
    new_positions: Vec<Vec2>,
    original_positions: Vec<&Vec2>,
    metadata: &KennelMetadata,
) -> Vec<Vec2> {
    let mut collision_lattice = CollisionLattice::from(metadata);
    let mut collision_count: usize = 0;

    for (new_position, original_position) in zip_eq(new_positions, &original_positions) {
        // check if the state is a non-moving one.
        if new_position.eq(original_position) {
            collision_lattice.add(new_position);
            continue;
        }

        // check for wall collisions
        if let Some(resolved_position) =
            resolve_wall_collision(&new_position, original_position, metadata)
        {
            collision_count += 1;
            collision_lattice.add(resolved_position);
            continue;
        }

        // check for creature collsions
        let collision_candidates = collision_lattice.get_collision_candidates(&new_position);
        if let Some(resolved_position) = collision_candidates
            .iter()
            .map(|collision_candidate| {
                resolve_creature_collision(
                    &new_position,
                    original_position,
                    collision_candidate,
                    metadata,
                )
            })
            .take_while_inclusive(|c| c.is_none())
            .last()
            .unwrap_or(None)
        {
            collision_count += 1;
            collision_lattice.add(resolved_position);
        }
    }

    match collision_count {
        0 => collision_lattice.into_vec(),
        _ => resolve_collisions(collision_lattice.into_vec(), original_positions, metadata),
    }
}

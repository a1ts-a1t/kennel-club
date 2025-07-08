use std::ops::IndexMut;

use itertools::{Itertools, multizip, zip_eq};
use rand::Rng;
use serde::Serialize;

use crate::{
    creature::{Creature, CreatureState},
    quadratic::{self, solve_quadratic},
    vec::Vec2,
};

#[derive(Serialize, Clone)]
pub struct KennelMetadata {
    width: f64,
    height: f64,
    creature_radius: f64, // half the side length of a creature collision box
    creature_count: u8,
}

struct CollisionLattice {
    width: usize,
    height: usize,
    lattice_distance: f64, // the distance between points on a lattice (horizontally or vertically)
    grid: Box<[Box<[Option<Vec<Vec2>>]>]>,
}

impl CollisionLattice {
    pub fn add(&mut self, position: Vec2) {
        let x_idx = (position.x / self.lattice_distance).floor() as usize;
        let y_idx = (position.y / self.lattice_distance).floor() as usize;

        let ele = self.grid.index_mut(x_idx).index_mut(y_idx);
        match ele {
            Some(v) => v.push(position),
            None => *ele = Some(vec![position]),
        };
    }

    pub fn get_collision_candidates(&self, position: &Vec2) -> Vec<Vec2> {
        let mut candidates: Vec<Vec2> = vec![];

        let x_idx = (position.x / self.lattice_distance).floor() as usize;
        let y_idx = (position.y / self.lattice_distance).floor() as usize;

        if let Some(ref v) = self.grid[x_idx][y_idx] {
            candidates.extend_from_slice(v);
        }

        // fetch surrounding tiles if they are in bounds.
        // i didn't want to incur the cost of extra abstractions
        // so here's a bunch of if statements. eat your heart out.
        let x_lower = x_idx > 0;
        let x_higher = x_idx < self.width - 1;
        let y_lower = y_idx > 0;
        let y_higher = y_idx < self.height - 1;

        if (x_lower && y_lower)
            && let Some(ref v) = self.grid[x_idx - 1][y_idx - 1]
        {
            candidates.extend_from_slice(v);
        }

        if (x_lower) && let Some(ref v) = self.grid[x_idx - 1][y_idx] {
            candidates.extend_from_slice(v);
        }

        if (x_lower && y_higher)
            && let Some(ref v) = self.grid[x_idx - 1][y_idx + 1]
        {
            candidates.extend_from_slice(v);
        }

        if (y_lower) && let Some(ref v) = self.grid[x_idx][y_idx - 1] {
            candidates.extend_from_slice(v);
        }

        if (y_higher) && let Some(ref v) = self.grid[x_idx][y_idx + 1] {
            candidates.extend_from_slice(v);
        }

        if (x_higher && y_lower)
            && let Some(ref v) = self.grid[x_idx + 1][y_idx - 1]
        {
            candidates.extend_from_slice(v);
        }

        if (x_higher) && let Some(ref v) = self.grid[x_idx + 1][y_idx] {
            candidates.extend_from_slice(v);
        }

        if (x_higher && y_higher)
            && let Some(ref v) = self.grid[x_idx + 1][y_idx + 1]
        {
            candidates.extend_from_slice(v);
        }

        candidates
    }

    pub fn as_vec(self) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = vec![];
        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            if let Some(ref positions) = self.grid[x][y] {
                v.extend_from_slice(positions);
            }
        }
        v
    }
}

impl From<&KennelMetadata> for CollisionLattice {
    fn from(metadata: &KennelMetadata) -> Self {
        let creature_size = 2f64 * metadata.creature_radius;
        let width = (metadata.width / creature_size).floor() as usize;
        let height = (metadata.height / creature_size).floor() as usize;
        let grid = vec![vec![None; height].into_boxed_slice(); width].into_boxed_slice();
        CollisionLattice {
            width,
            height,
            lattice_distance: creature_size,
            grid,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Kennel {
    metadata: KennelMetadata,
    creatures: Vec<Creature>,
}

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
        quadratic::RealQuadraticRoots::Double(root, _) if root > 0f64 && root < 1f64 => root,
        quadratic::RealQuadraticRoots::Double(_, root) if root > 0f64 && root < 1f64 => root,
        quadratic::RealQuadraticRoots::Single(root) if root > 0f64 && root < 1f64 => root,
        _ => 0f64,
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

fn resolve_collisions(
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
        0 => collision_lattice.as_vec(),
        _ => resolve_collisions(collision_lattice.as_vec(), original_positions, metadata),
    }
}

impl Kennel {
    pub fn new() -> Kennel {
        todo!("Blue noise initialization");
    }

    pub fn next<R: Rng + ?Sized>(self, rng: &mut R) -> Kennel {
        let (states, positions, metadatas): (Vec<_>, Vec<_>, Vec<_>) = self
            .creatures
            .iter()
            .map(|c| (&c.state, &c.position, &c.metadata))
            .multiunzip();

        let (new_states, new_positions): (Vec<_>, Vec<_>) = zip_eq(&states, &positions)
            .map(|(state, position)| {
                let new_state = state.next(rng);
                let new_position = match &new_state {
                    CreatureState::Flee(_) => todo!(),
                    CreatureState::Follow(_) => todo!(),
                    _ => (*position).clone(),
                };
                (new_state, new_position)
            })
            .unzip();

        let new_positions = resolve_collisions(new_positions, positions, &self.metadata);
        let new_creatures: Vec<Creature> = multizip((new_states, new_positions, metadatas))
            .map(|(state, position, metadata)| Creature {
                state,
                position,
                metadata: metadata.clone(),
            })
            .collect();

        Kennel {
            metadata: self.metadata,
            creatures: new_creatures,
        }
    }
}

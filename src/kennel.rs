use std::cmp::{Ordering, min};

use itertools::{Itertools, multizip, zip_eq};
use rand::{Rng, rngs::ThreadRng};
use serde::Serialize;

use crate::{
    creature::{Creature, CreatureState},
    vec::Vec2,
};

#[derive(Serialize, Clone)]
pub struct KennelMetadata {
    width: f64,
    height: f64,
    creature_radius: f64, // half the side length of a creature collision box
    creature_count: u8,
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
    let creature_size = 2f64 * metadata.creature_radius;
    let t_x: Option<f64> = match new_position.x.partial_cmp(&check_position.x).unwrap() {
        Ordering::Less if (check_position.x - new_position.x) < creature_size => Some(
            (check_position.x - creature_size - original_position.x)
                / (new_position.x - original_position.x),
        ),
        Ordering::Equal => Some(0f64),
        Ordering::Greater if (new_position.x - check_position.x) < creature_size => Some(
            (check_position.x + creature_size - original_position.x)
                / (new_position.x - original_position.x),
        ),
        _ => None, // no collision
    };

    if t_x.is_none() {
        return None;
    }

    let t_y: Option<f64> = match new_position.y.partial_cmp(&check_position.y).unwrap() {
        Ordering::Less if (check_position.y - new_position.y) < creature_size => Some(
            (check_position.y - creature_size - original_position.y)
                / (new_position.y - original_position.y),
        ),
        Ordering::Equal => Some(0f64),
        Ordering::Greater if (new_position.y - check_position.y) < creature_size => Some(
            (check_position.y + creature_size - original_position.y)
                / (new_position.y - original_position.y),
        ),
        _ => None, // no collision
    };

    if t_y.is_none() {
        return None;
    }

    let t = f64::min(t_x.unwrap(), t_y.unwrap());
    let delta = new_position - original_position;
    Some(Vec2 {
        x: original_position.x + delta.x * t,
        y: original_position.y + delta.y * t,
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
    } else if new_position.x >= (metadata.width - metadata.creature_radius) {
        let delta_x = new_position.x - original_position.x;
        t = (metadata.width - metadata.creature_radius - original_position.x) / delta_x
    }

    if new_position.y < metadata.creature_radius {
        let delta_y = new_position.y - original_position.y;
        t = f64::min(
            (metadata.creature_radius - original_position.y) / delta_y,
            t,
        );
    } else if new_position.y >= (metadata.height - metadata.creature_radius) {
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
    let mut resolved_positions: Vec<Vec2> = vec![];
    let mut collision_count: usize = 0;

    for (new_position, original_position) in zip_eq(new_positions, &original_positions) {
        // check if the state is a non-moving one.
        if new_position.eq(original_position) {
            resolved_positions.push(new_position);
            continue;
        }

        // check for wall collisions
        if let Some(resolved_position) =
            resolve_wall_collision(&new_position, original_position, metadata)
        {
            collision_count += 1;
            resolved_positions.push(resolved_position);
            continue;
        }

        // todo: we're checking for collisions against all previously checked members
        //       implement a more efficient way to check for collision than
        //       just iterating through using a proximity grid
        for check_position in &resolved_positions {
            if let Some(resolved_position) = resolve_creature_collision(
                &new_position,
                original_position,
                check_position,
                metadata,
            ) {
                collision_count += 1;
                resolved_positions.push(resolved_position);
                break;
            }
        }
    }

    match collision_count {
        0 => resolved_positions,
        _ => resolve_collisions(resolved_positions, original_positions, metadata),
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

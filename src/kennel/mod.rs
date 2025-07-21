mod collision_lattice;
mod collision_resolvers;
mod metadata;

use itertools::{Itertools, multizip, zip_eq};
use rand::{Rng, distr::Uniform};
use serde::Serialize;

use crate::{
    creature::{Creature, CreatureMetadata, CreatureState},
    kennel::{collision_lattice::CollisionLattice, collision_resolvers::resolve_collisions, metadata::KennelMetadata},
    math::Vec2,
    quadtree::Quadtree,
};

#[derive(Serialize, Clone)]
pub struct Kennel {
    metadata: KennelMetadata,
    creatures: Vec<Creature>,
}

impl Kennel {
    pub fn new<R: Rng + ?Sized>(
        rng: &mut R,
        creature_metadatas: Vec<CreatureMetadata>,
        metadata: KennelMetadata,
    ) -> Kennel {
        // ok, we're doing dart throwing. don't be mad at me. i want unbiased non-maximal coverage so that rules
        // out Bridson's algorithm, and realistically, n should always be small enough (wrt kennel size) that this
        // isn't really a problem.
        // if it is, we'll tackle it when we get there

        let mut collision_lattice = CollisionLattice::from(&metadata);
        let distr_x = Uniform::new(0f64, metadata.width).unwrap();
        let distr_y = Uniform::new(0f64, metadata.height).unwrap();
        let desired_squared_norm = 4f64 * metadata.creature_radius * metadata.creature_radius;
        while collision_lattice.count < metadata.creature_count {
            let test_position = Vec2 {
                x: rng.sample(distr_x),
                y: rng.sample(distr_y),
            };
            let has_collision = collision_lattice
                .get_collision_candidates(&test_position)
                .iter()
                .any(|candidate_position| {
                    (&test_position - candidate_position).squared_norm() < desired_squared_norm
                });

            if !has_collision {
                collision_lattice.add(test_position);
            }
        }

        let positions = collision_lattice.into_vec();
        let creatures: Vec<_> = zip_eq(creature_metadatas, positions)
            .map(|(creature_metadata, position)| Creature {
                metadata: creature_metadata,
                position,
                state: rng.random(),
            })
            .collect();

        Kennel {
            metadata,
            creatures,
        }
    }

    pub fn next<R: Rng + ?Sized>(self, rng: &mut R) -> Kennel {
        let (states, positions, metadatas): (Vec<_>, Vec<_>, Vec<_>) = self
            .creatures
            .iter()
            .map(|c| (&c.state, &c.position, &c.metadata))
            .multiunzip();

        let quadtree = Quadtree::from_data(&positions, self.metadata.width, self.metadata.height);
        let (new_states, new_positions): (Vec<_>, Vec<_>) = zip_eq(&states, &positions)
            .map(|(state, position)| {
                let new_state = state.next(rng);
                let new_position = match &new_state {
                    CreatureState::Flee(_) => match quadtree.get_closest(position) {
                        Some(closest_position) => self.metadata.step_size * (*position - &closest_position).normalize(),
                        None => self.metadata.step_size * rng.random::<Vec2>(),
                    },
                    CreatureState::Follow(_) => match quadtree.get_closest(position) {
                        Some(closest_position) => self.metadata.step_size * (&closest_position - *position).normalize(),
                        None => self.metadata.step_size * rng.random::<Vec2>(),
                    },
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

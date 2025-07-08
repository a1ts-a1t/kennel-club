use rand::{rngs::ThreadRng, Rng};
use serde::Serialize;
use itertools::{multizip, zip_eq, Itertools};

use crate::{creature::{Creature, CreatureState}, vec::Vec2};

#[derive(Serialize, Clone)]
pub struct KennelMetadata {
    creature_size: f64, // side length of a square collision box
    creature_count: u8,
}

#[derive(Serialize, Clone)]
pub struct Kennel {
    metadata: KennelMetadata,
    creatures: Vec<Creature>,
}

fn resolve_collisions(new_positions: Vec<Vec2>, original_positions: &Vec<&Vec2>, creature_size: f64) -> Vec<Vec2> {
    todo!();
}

impl Kennel {
    pub fn new() -> Kennel {
        todo!("Blue noise initialization");
    }

    pub fn next<R: Rng + ?Sized>(self, rng: &mut R) -> Kennel {
        // todo: we're checking for collisions against all members of this set
        //       implement a more efficient way to check for collision than
        //       just iterating through.
        let (states, positions, metadatas): (Vec<_>, Vec<_>, Vec<_>) = self.creatures.iter()
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

        let new_positions = resolve_collisions(new_positions, &positions, self.metadata.creature_size);
        let new_creatures: Vec<Creature> = multizip((new_states, new_positions, metadatas))
            .map(|(state, position, metadata)| {
                Creature { state, position, metadata: metadata.clone() }
            })
            .collect();

        Kennel {
            metadata: self.metadata,
            creatures: new_creatures,
        }
    }
}

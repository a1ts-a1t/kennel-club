use std::iter::zip;

use itertools::Itertools;
use rand::Rng;
use termion::terminal_size;

use crate::creature::Creature;
use crate::creature::Metadata;
use crate::kennel::collision::Arena;
use crate::math::Vec2;
use crate::physics::Collidable;

mod collision;

pub struct Kennel {
    creatures: Vec<Creature>,
}

static MAX_INITIALIZATION_RETRIES: u8 = 32;

impl Kennel {
    /**
     * Initialize a new kennel and reposition creatures such that no two are colliding
     * and none are colliding within the walls.
     *
     * I know dart throwing is not sexy, but give me a break, this is like. n=10 or something.
     */
    pub fn new<R: Rng + ?Sized>(
        creature_metadata: Vec<Metadata>,
        rng: &mut R,
    ) -> Result<Self, String> {
        let creatures: Vec<_> = creature_metadata
            .into_iter()
            .map(|metadata| Creature::from_metadata(metadata, rng))
            .collect();

        let mut unpositioned_creatures = creatures.into_iter();
        let mut repositioned_creatures: Vec<Creature> = vec![];

        while let Some(current_creature) = unpositioned_creatures.next() {
            let radius = current_creature.metadata.radius;
            let diameter = radius * 2.0;
            if diameter > 1.0 {
                return Err(format!(
                    "Creature {} has radius {} and is too large for the kennel size.",
                    current_creature.metadata.id, radius
                ));
            }

            let random_collidable = |_| {
                let position = Vec2::new(
                    rng.random_range(f64::next_up(radius)..(1.0 - radius)),
                    rng.random_range(f64::next_up(radius)..(1.0 - radius)),
                );
                Collidable::new(position, radius)
            };

            let is_not_colliding = |collidable: &Collidable| {
                !repositioned_creatures
                    .iter()
                    .any(|creature| collidable.is_colliding(&creature.as_collidable()))
            };

            let repositioned_collidable = (0..MAX_INITIALIZATION_RETRIES)
                .into_iter()
                .map(random_collidable)
                .find(is_not_colliding);

            match repositioned_collidable {
                Some(c) => repositioned_creatures.push(current_creature.set_position(c.position)),
                None => {
                    return Err(format!(
                        "Unable to position creature {}",
                        current_creature.metadata.id
                    ));
                }
            }
        }

        Ok(Kennel {
            creatures: repositioned_creatures,
        })
    }

    pub fn center_of_mass(&self) -> Vec2 {
        let weighted_position_sum = self
            .creatures
            .iter()
            .map(|creature| &creature.radius() * &creature.position)
            .reduce(|acc, e| acc + e);

        let weight_sum = self
            .creatures
            .iter()
            .map(|creature| creature.radius())
            .reduce(|acc, e| acc + e);

        match (weighted_position_sum, weight_sum) {
            (Some(n), Some(d)) => &n / d,
            _ => Vec2 { x: 0.5, y: 0.5 },
        }
    }

    /**
     * Consumes the current kennel state and creates
     * a kennel that is in the next time state.
     * This moves each creature forward a time step
     * and de-collides them.
     */
    pub fn next<R: Rng + ?Sized>(self, rng: &mut R) -> Result<Self, String> {
        // get the new states of all creatures
        let center_of_mass = self.center_of_mass();

        let new_creatures: Vec<_> = self
            .creatures
            .into_iter()
            .map(|creature| creature.into_next_state(rng))
            .collect();

        // create new steps and resolve to bounds
        let mut arena: Arena = Arena::new();
        for creature in new_creatures.iter() {
            let step = creature.get_next_step(&center_of_mass, rng);
            arena.add(step);
        }

        let steps = arena.into_vec();
        let repositioned_creatures: Vec<_> = zip(new_creatures, steps)
            .map(|(creature, step)| creature.step(step))
            .collect();

        Ok(Kennel {
            creatures: repositioned_creatures,
        })
    }

    /**
     * Prints the kennel out to the terminal.
     * Each terminal cell represents a chunk of the kennel.
     * Each terminal cell will display the number of creatures in that cell.
     * If the number of creatures is greater than 9, it will display `+`.
     */
    #[allow(dead_code)]
    pub fn pretty_print(&self) -> () {
        let (screen_width, screen_height) = terminal_size().unwrap();
        let cell_width = 1.0 / Into::<f64>::into(screen_width);
        let cell_height = 1.0 / Into::<f64>::into(screen_height);

        // bucket the creatures by screen position
        // and count the number of creatures at each position
        let creature_screen_positions = self
            .creatures
            .iter()
            .map(|creature| {
                let position = creature.position.clone();
                let idx = (position.x / cell_width).floor() as u16;
                let idy = (position.y / cell_height).floor() as u16;
                idy * screen_width + idx
            })
            .counts();

        print!("{esc}c", esc = 27 as char); // clear the screen
        for index in 0..screen_width * screen_height {
            if index % screen_width == 0 && index > 0 {
                print!("\n");
            }

            match creature_screen_positions.get(&index) {
                Some(count) if *count >= 10 => print!("+"),
                Some(count) => print!("{}", count),
                None => print!("·"),
            }
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) -> () {
        print!("{esc}c", esc = 27 as char); // clear the screen
        for creature in self.creatures.iter() {
            println!(
                "{:5}({}, {}) - {:?}",
                creature.metadata.id,
                creature.position.x,
                creature.position.y,
                creature.creature_state
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::SmallRng};

    static RNG_SEED: u64 = 1;

    #[test]
    fn test_new_fail() {
        let mut rng = SmallRng::seed_from_u64(RNG_SEED);
        let metadata = Metadata::new("id".to_string(), 0.0, 100.0, None);
        let kennel_result = Kennel::new(vec![metadata], &mut rng);
        assert!(kennel_result.is_err());
    }

    #[test]
    fn test_new_collisions() {
        let mut rng = SmallRng::seed_from_u64(RNG_SEED);
        let metadata: Vec<_> = (1..=10)
            .into_iter()
            .map(|radius| {
                Metadata::new(format!("id{}", radius), 0.0, (radius as f64) / 100.0, None)
            })
            .collect();

        let kennel = Kennel::new(metadata, &mut rng).unwrap();
        let collidable_combinations = kennel
            .creatures
            .into_iter()
            .map(|creature| creature.as_collidable())
            .combinations(2);

        for collidable_combination in collidable_combinations {
            let (c1, c2) = (
                collidable_combination.get(0).unwrap(),
                collidable_combination.get(1).unwrap(),
            );
            if c1.is_colliding(c2) {
                panic!("Pairwise collision found during initialization");
            }
        }
    }
}

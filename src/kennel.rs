use std::iter::zip;

use itertools::Itertools;
use rand::Rng;
use termion::terminal_size;

use crate::creature::Creature;
use crate::creature::Metadatum;
use crate::creature::StateType;
use crate::math::Vec2;
use crate::physics::{Collidable, Step, StepResolutionResult};

pub struct Kennel {
    creatures: Vec<Creature>,
}

static MAX_INITIALIZATION_RETRIES: u8 = 32;

/**
 * Returns a random non-inclusive value between low and high. That is
 * a uniformly distributed random value in (low, high)
 *
 * I KNOW THE DISTRIBUTION IS NOT TECHNICALLY UNIFORM
 * I DON'T CARE
 */
fn random_noninclusive<R: Rng + ?Sized>(low: f64, high: f64, rng: &mut R) -> f64 {
    let val = rng.random_range(low..high);
    if val == low {
        return low.next_up();
    }
    val
}

impl Kennel {
    /**
     * Initialize a new kennel and reposition creatures such that no two are colliding
     * and none are colliding within the walls.
     *
     * I know dart throwing is not sexy, but give me a break, this is like. n=10 or something.
     */
    pub fn new<R: Rng + ?Sized>(
        creature_metadata: Vec<Metadatum>,
        rng: &mut R,
    ) -> Result<Self, String> {
        let creatures: Vec<_> = creature_metadata
            .into_iter()
            .map(|metadatum| Creature::from_metadatum(metadatum, rng))
            .collect();

        let mut unpositioned_creatures = creatures.into_iter();
        let mut repositioned_creatures: Vec<Creature> = vec![];

        while let Some(current_creature) = unpositioned_creatures.next() {
            let radius = current_creature.metadatum.radius;
            let diameter = radius * 2.0;
            if diameter > 1.0 {
                return Err(format!(
                    "Creature {} has radius {} and is too large for the kennel size.",
                    current_creature.metadatum.id, radius
                ));
            }

            let random_collidable = |_| {
                let position: Vec2 = (
                    random_noninclusive(radius, 1.0 - radius, rng),
                    random_noninclusive(radius, 1.0 - radius, rng),
                )
                    .into();
                Collidable::new(position, radius)
            };

            let is_not_colliding = |collidable: &Collidable| {
                !repositioned_creatures
                    .iter()
                    .any(|creature| collidable.is_colliding(&creature.collidable))
            };

            let repositioned_collidable = (0..MAX_INITIALIZATION_RETRIES)
                .into_iter()
                .map(random_collidable)
                .find(is_not_colliding);

            match repositioned_collidable {
                Some(c) => repositioned_creatures.push(current_creature.reposition(c.position)),
                None => {
                    return Err(format!(
                        "Unable to position creature {}",
                        current_creature.metadatum.id
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
            .map(|creature| &creature.collidable.radius * &creature.collidable.position)
            .reduce(|acc, e| acc + e);

        let weight_sum = self
            .creatures
            .iter()
            .map(|creature| creature.collidable.radius)
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
    pub fn next<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Self, String> {
        // get the new states of all creatures
        let center_of_mass = self.center_of_mass();

        let new_creatures: Vec<_> = self
            .creatures
            .iter()
            .map(|creature| creature.next_state(rng))
            .collect();

        let to_step = |creature: &Creature| {
            let collidable = creature.collidable.clone();
            match creature.state.state_type {
                StateType::Follow => {
                    let delta = &center_of_mass - &collidable.position;
                    Step::new(collidable, delta.with_norm(creature.metadatum.step_size))
                }
                StateType::Flee => {
                    let delta = &collidable.position - &center_of_mass;
                    Step::new(collidable, delta.with_norm(creature.metadatum.step_size))
                }
                _ => Step::new(collidable, Vec2::zero()),
            }
        };

        // create new steps and resolve to bounds
        let mut steps: Vec<Step> = vec![];
        for creature in new_creatures.iter() {
            let unresolved_step = to_step(creature);
            let resolution_result = unresolved_step.resolve_to_unit_bounds();
            match resolution_result {
                StepResolutionResult::Ok => steps.push(unresolved_step),
                StepResolutionResult::ResolvedTo(step) => steps.push(step),
                StepResolutionResult::Err => {
                    return Err(format!("Out of bounds: ({:?})", creature.collidable));
                }
            }
        }

        // resolve collisions between collidables
        // I KNOW CHECKING ALL PAIRS IS NEITHER EFFICIENT NOR COOL
        // BUT AGAIN LIKE N<10
        // (something something muttering quadtree under my breath)

        let mut recheck = true;
        while recheck {
            recheck = false;
            for idxs in (0..steps.len()).combinations(2) {
                let (i, j) = (idxs[0], idxs[1]);

                let step_i = steps.get(i).unwrap();
                let step_j = steps.get(j).unwrap();

                match Step::resolve_steps(step_i, step_j) {
                    StepResolutionResult::Ok => (),
                    StepResolutionResult::ResolvedTo((resolved_step_i, resolved_step_j)) => {
                        recheck = true;
                        steps[i] = resolved_step_i;
                        steps[j] = resolved_step_j;
                    }
                    StepResolutionResult::Err => {
                        return Err(format!("Unresolved collision: ({:?},{:?})", step_i, step_j));
                    }
                };
            }
        }

        let repositioned_creatures: Vec<_> = zip(new_creatures, steps)
            .map(|(creature, step)| creature.reposition(step.collapse().position))
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
                let position = creature.position();
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

    pub fn print(&self) -> () {
        print!("{esc}c", esc = 27 as char); // clear the screen
        for creature in self.creatures.iter() {
            println!(
                "{:5}({:.3}, {:.3}) - {:?}",
                creature.metadatum.id,
                creature.position().x,
                creature.position().y,
                creature.state.state_type
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
        let metadatum = Metadatum::new("id".to_string(), 0.0, 100.0);
        let kennel_result = Kennel::new(vec![metadatum], &mut rng);
        assert!(kennel_result.is_err());
    }

    #[test]
    fn test_new_collisions() {
        let mut rng = SmallRng::seed_from_u64(RNG_SEED);
        let metadata: Vec<_> = (1..=10)
            .into_iter()
            .map(|radius| Metadatum::new(format!("id{}", radius), 0.0, (radius as f64) / 100.0))
            .collect();

        let kennel = Kennel::new(metadata, &mut rng).unwrap();
        let collidable_combinations = kennel
            .creatures
            .into_iter()
            .map(|creature| creature.collidable)
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

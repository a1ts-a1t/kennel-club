use itertools::Itertools;
use rand::Rng;
use termion::terminal_size;

use crate::collidable::Collidable;
use crate::creature::Creature;
use crate::creature::Metadatum;
use crate::vec2::Position;

pub struct Kennel {
    width: f64,
    height: f64,
    creatures: Vec<Creature>,
}

static MAX_INITIALIZATION_RETRIES: u8 = 32;

/**
 * Returns a random non-inclusive value between low and high. That is
 * a uniformly distributed random value in (low, high)
 */
fn random_noninclusive<R: Rng + ?Sized>(low: f64, high: f64, rng: &mut R) -> f64 {
    let val1 = rng.random_range(low..high);
    let val2 = rng.random_range(-high..-low);
    (val1 - val2) / 2.0
}

impl Kennel {
    /**
     * Initialize a new kennel and reposition creatures such that no two are colliding
     * and none are colliding within the walls.
     *
     * I know dart throwing is not sexy, but give me a break, this is like. n=10 or something.
     */
    pub fn new<R: Rng + ?Sized>(
        width: f64,
        height: f64,
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
            if diameter >= width || diameter >= height {
                return Err(format!(
                    "Creature {} has radius {} and is too large for the kennel size.",
                    current_creature.metadatum.id, radius
                ));
            }

            let random_collidable = |_| {
                let position: Position = (
                    random_noninclusive(radius, width - radius, rng),
                    random_noninclusive(radius, height - radius, rng),
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
            width,
            height,
            creatures: repositioned_creatures,
        })
    }

    /**
     * Consumes the current kennel state and creates
     * a kennel that is in the next time state.
     * This moves each creature forward a time step
     * and de-collides them.
     */
    pub fn next(self) -> Self {
        // get the new states of all creatures
        //
        // reposition all creatures
        //
        // resolve the collisions for all creatures
        //
        // recompute the center of mass of all creatures
        todo!();
    }

    /**
     * Returns a kennel that matches the specified width and height.
     * scales positions for all creatures and scales collidable radii.
     */
    pub fn resize(&self, width: f64, height: f64) -> Self {
        todo!();
    }

    /**
     * Prints the kennel out to the terminal.
     * Each terminal cell represents a chunk of the kennel.
     * Each terminal cell will display the number of creatures in that cell.
     * If the number of creatures is greater than 9, it will display `+`.
     */
    pub fn print(&self) -> () {
        let (screen_width, screen_height) = terminal_size().unwrap();
        let cell_width = self.width / Into::<f64>::into(screen_width);
        let cell_height = self.height / Into::<f64>::into(screen_height);

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
        let kennel_result = Kennel::new(10.0, 10.0, vec![metadatum], &mut rng);
        assert!(kennel_result.is_err());
    }

    #[test]
    fn test_new_collisions() {
        let mut rng = SmallRng::seed_from_u64(RNG_SEED);
        let metadata: Vec<_> = (1..=10)
            .into_iter()
            .map(|radius| Metadatum::new(format!("id{}", radius), 0.0, radius as f64))
            .collect();

        let kennel = Kennel::new(500.0, 500.0, metadata, &mut rng).unwrap();
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
                let position_diff = &c1.position - &c2.position;
                println!("Collidable1: [{:?}]", c1);
                println!("Collidable2: [{:?}]", c2);
                println!("PositionDiff: [{:?}]", position_diff);
                println!("Diff squared norm: [{}]", position_diff.squared_norm());
                panic!("Pairwise collision found during initialization");
            }
        }
    }
}

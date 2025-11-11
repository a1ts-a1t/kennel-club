use std::io::{BufWriter, Cursor};
use std::iter::zip;
use std::path::Path;

use image::imageops::overlay;
use image::{DynamicImage, ImageFormat, RgbaImage};
use itertools::Itertools;
use rand::Rng;
use termion::terminal_size;

use crate::creature::{self, Creature};
use crate::kennel::collision::Arena;
use crate::math::Vec2;
use crate::physics::Collidable;
use crate::{Sprite, sprite};

mod collision;

pub struct Kennel {
    creatures: Vec<Creature>,
}

static MAX_INITIALIZATION_RETRIES: u8 = 32;

impl Kennel {
    pub fn load<R: Rng + ?Sized>(dir: &Path, rng: &mut R) -> Result<Self, String> {
        let json = std::fs::read_to_string(dir.join("metadata.json"))
            .map_err(|_| "Unable to read metadata file")?;

        let metadatas: Vec<creature::Metadata> = serde_json::from_str(&json)
            .map_err(|e| format!("Unable to deserialize creature metadata. {}", e))?;

        let creatures: Vec<Creature> = metadatas
            .into_iter()
            .map(|metadata| Creature::load(metadata, dir))
            .collect();

        Kennel::new(creatures, rng)
    }

    /**
     * Initialize a new kennel and reposition creatures such that no two are colliding
     * and none are colliding within the walls.
     * I know dart throwing is not sexy, but give me a break, this is like. n=10 or something.
     */
    pub fn new<R: Rng + ?Sized>(creatures: Vec<Creature>, rng: &mut R) -> Result<Self, String> {
        let mut repositioned_creatures: Vec<Creature> = vec![];
        for current_creature in creatures.into_iter() {
            let radius = current_creature.radius;
            let diameter = radius * 2.0;
            if diameter > 1.0 {
                return Err(format!(
                    "Creature {} has radius {} and is too large for the kennel size.",
                    current_creature.id, radius
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
                .map(random_collidable)
                .find(is_not_colliding);

            match repositioned_collidable {
                Some(c) => repositioned_creatures.push(current_creature.set_position(c.position)),
                None => {
                    return Err(format!(
                        "Unable to position creature {}",
                        current_creature.id
                    ));
                }
            }
        }

        Ok(Kennel {
            creatures: repositioned_creatures,
        })
    }

    fn center_of_mass(&self) -> Vec2 {
        if self.creatures.len() <= 1 {
            return Vec2 { x: 0.5, y: 0.5 };
        }

        let weighted_position_sum = self
            .creatures
            .iter()
            .map(|creature| creature.radius * &creature.position)
            .reduce(|acc, e| acc + e)
            .expect("Error computing center of mass");

        let weight_sum = self
            .creatures
            .iter()
            .map(|creature| creature.radius)
            .reduce(|acc, e| acc + e)
            .expect("Error computing center of mass");

        &weighted_position_sum / weight_sum
    }

    /**
     * creates a kennel that is in the next time step.
     * This moves each creature forward a time step
     * and de-collides them.
     */
    pub fn next<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<Self, String> {
        let center_of_mass = self.center_of_mass();
        let new_creatures: Vec<_> = self
            .creatures
            .iter()
            .map(|creature| creature.with_next_state(rng))
            .collect();

        let mut arena: Arena = Arena::new();
        for creature in new_creatures.iter() {
            let step = creature.get_next_step(&center_of_mass);
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

    pub fn creatures(&self) -> Vec<&Creature> {
        self.creatures.iter().collect()
    }

    /**
     * Prints the kennel out to the terminal.
     * Each terminal cell represents a chunk of the kennel.
     * Each terminal cell will display the number of creatures in that cell.
     * If the number of creatures is greater than 9, it will display `+`.
     */
    pub fn pretty_print(&self) {
        let (screen_width, screen_height) = terminal_size().unwrap();
        let cell_width = 1.0 / Into::<f64>::into(screen_width);
        let cell_height = 1.0 / Into::<f64>::into(screen_height);

        // bucket the creatures by screen position
        // and count the number of creatures at each position
        let creature_screen_positions = self
            .creatures
            .iter()
            .map(|creature| {
                let position = creature.position;
                let idx = (position.x / cell_width) as u16;
                let idy = (position.y / cell_height) as u16;
                idy * screen_width + idx
            })
            .counts();

        print!("{esc}c", esc = 27 as char); // clear the screen
        for index in 0..screen_width * screen_height {
            if index % screen_width == 0 && index > 0 {
                println!();
            }

            match creature_screen_positions.get(&index) {
                Some(count) if *count >= 10 => print!("+"),
                Some(count) => print!("{}", count),
                None => print!("·"),
            }
        }
    }

    pub fn print(&self) {
        print!("{esc}c", esc = 27 as char); // clear the screen
        for creature in self.creatures.iter() {
            println!(
                "{:5}({}, {}) - {:?}",
                creature.id, creature.position.x, creature.position.y, creature.creature_state
            );
        }
    }

    pub fn get_sprite(&self, id: &str) -> Option<&Sprite> {
        self.creatures
            .iter()
            .find(|creature| creature.id == id)
            .map(|creature| creature.sprite())
    }

    pub fn get_sprite_by(
        &self,
        id: &str,
        sprite_state: &sprite::State,
        frame: &usize,
    ) -> Option<&Sprite> {
        self.creatures
            .iter()
            .find(|creature| creature.id == id)
            .map(|creature| creature.sprite_sheet.get_sprite(sprite_state, *frame))
    }

    pub fn get_image(
        &self,
        canvas_width: u32,
        canvas_height: u32,
        image_format: ImageFormat,
    ) -> Result<Vec<u8>, String> {
        let mut canvas = RgbaImage::new(canvas_width, canvas_height);
        let canvas_scale_factor = u32::min(canvas_width, canvas_height) as f64;

        for creature in self.creatures() {
            let sprite = creature.sprite();

            // scale creature sprite
            let sprite_scale_factor =
                2.0 * creature.radius * canvas_scale_factor * sprite.scale_factor();
            let image = sprite.get_scaled_image(sprite_scale_factor);

            // get canvas position, WRT canvas pixel units
            let canvas_position = canvas_scale_factor * &creature.position - &creature.radius;

            let x_start = (canvas_position.x as u32).clamp(0, canvas_width - image.width() - 1);
            let y_start = (canvas_position.y as u32).clamp(0, canvas_height - image.height() - 1);

            let image_buffer = image
                .as_rgba8()
                .ok_or("Error representing sprite with 32bit RGBA")?;
            overlay(&mut canvas, image_buffer, x_start.into(), y_start.into());
        }

        let canvas = DynamicImage::from(canvas);
        let mut image_buffer = BufWriter::new(Cursor::new(Vec::new()));
        canvas
            .write_to(&mut image_buffer, image_format)
            .map_err(|_| "Error writing kennel image")?;

        let bytes = image_buffer
            .into_inner()
            .map(Cursor::into_inner)
            .map_err(|_| "Error writing kennel image to bytes")?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature::Metadata;
    use rand::{SeedableRng, rngs::SmallRng};

    static RNG_SEED: u64 = 1;

    #[test]
    fn test_new_fail() {
        let mut rng = SmallRng::seed_from_u64(RNG_SEED);
        let creature: Creature = Metadata::mock(100.0).into();
        let kennel_result = Kennel::new(vec![creature], &mut rng);
        assert!(kennel_result.is_err());
    }

    #[test]
    fn test_new_collisions() {
        let mut rng = SmallRng::seed_from_u64(RNG_SEED);
        let metadata: Vec<_> = (1..=10)
            .into_iter()
            .map(|radius| Metadata::mock((radius as f64) / 100.0).into())
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

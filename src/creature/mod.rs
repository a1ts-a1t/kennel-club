use std::path::Path;

use image::DynamicImage;
pub use metadata::Metadata;
use rand::Rng;
use state::State;

use crate::physics::Step;
use crate::sprite;
use crate::{math::Vec2, physics::Collidable};

mod metadata;
mod state;

#[derive(Debug)]
pub struct Creature {
    pub id: String,
    pub step_size: f64,
    pub radius: f64,
    pub url: String,
    pub creature_state: State,
    pub position: Vec2,
    pub sprite_state: sprite::State,
    pub sprite_state_duration: usize,
    pub sprite_sheet: sprite::Sheet,
}

#[cfg(test)]
impl From<Metadata> for Creature {
    fn from(metadata: Metadata) -> Self {
        let sprite_sheet = sprite::Sheet::new();
        Creature {
            id: metadata.id,
            radius: metadata.radius,
            step_size: metadata.step_size,
            url: metadata.url,
            creature_state: metadata.initial_state,
            position: Vec2::zero(),
            sprite_state: sprite::State::Idle,
            sprite_state_duration: 0,
            sprite_sheet,
        }
    }
}

impl Creature {
    pub fn load(metadata: Metadata, data_dir: &Path) -> Self {
        let sprite_sheet = metadata.sprite_loader.load(&data_dir.join(&metadata.id));
        Creature {
            id: metadata.id,
            radius: metadata.radius,
            step_size: metadata.step_size,
            url: metadata.url,
            creature_state: metadata.initial_state,
            position: Vec2::zero(),
            sprite_state: sprite::State::Idle,
            sprite_state_duration: 0,
            sprite_sheet,
        }
    }

    /**
     * Computes the next state (randomly) for the creature.
     * DOES NOT REPOSITION THE CREATURE. THE COLLIDABLE DOES NOT CHANGE.
     * THE SPRITE STATE DOES NOT CHANGE.
     */
    pub fn with_next_state<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let next_state = self.creature_state.next(rng);
        Creature {
            id: self.id.clone(),
            radius: self.radius,
            step_size: self.step_size,
            url: self.url.clone(),
            creature_state: next_state,
            position: self.position,
            sprite_state: self.sprite_state,
            sprite_state_duration: self.sprite_state_duration,
            sprite_sheet: self.sprite_sheet.clone(),
        }
    }

    /**
     * Has the creature take a step in the direction.
     * Changes the sprite.
     */
    pub fn step(self, step: Step) -> Self {
        let new_sprite_state = match sprite::State::from_delta(&step.delta) {
            Some(s) => s,
            None if self.creature_state == State::Sleep => sprite::State::Sleep,
            None => sprite::State::Idle,
        };

        let new_sprite_state_duration = if new_sprite_state == self.sprite_state {
            self.sprite_state_duration + 1
        } else {
            0
        };

        let new_position = step.resolve().position;
        Creature {
            id: self.id,
            radius: self.radius,
            step_size: self.step_size,
            creature_state: self.creature_state,
            url: self.url,
            position: new_position,
            sprite_state: new_sprite_state,
            sprite_state_duration: new_sprite_state_duration,
            sprite_sheet: self.sprite_sheet,
        }
    }

    /**
     * Set position field WITHOUT CHANGING ANYTHING ELSE
     */
    pub fn set_position(self, position: Vec2) -> Self {
        Creature {
            id: self.id,
            radius: self.radius,
            step_size: self.step_size,
            url: self.url,
            creature_state: self.creature_state,
            position,
            sprite_state: self.sprite_state,
            sprite_state_duration: self.sprite_state_duration,
            sprite_sheet: self.sprite_sheet,
        }
    }

    /**
     * Calculates the next step given the creature's position
     * and a center of mass to trend toward.
     */
    pub fn get_next_step(&self, center_of_mass: &Vec2) -> Step {
        match self.creature_state {
            State::Follow => {
                let delta = center_of_mass - &self.position;
                Step::new(self.as_collidable(), delta.with_norm(self.step_size))
            }
            State::Flee => {
                let delta = &self.position - center_of_mass;
                Step::new(self.as_collidable(), delta.with_norm(self.step_size))
            }
            _ => Step::new(self.as_collidable(), Vec2::zero()),
        }
    }

    pub fn as_collidable(&self) -> Collidable {
        Collidable::new(self.position, self.radius)
    }

    pub fn sprite(&self) -> &DynamicImage {
        self.sprite_sheet
            .get_sprite(&self.sprite_state, self.sprite_state_duration)
    }
}

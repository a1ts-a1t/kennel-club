pub use metadata::Metadata;
use rand::Rng;
use state::CreatureState;

use crate::physics::Step;
use crate::sprite::SpriteState;
use crate::{math::Vec2, physics::Collidable};

mod metadata;
mod state;

static JITTER_STRENGTH: f64 = 0.2;

#[derive(Debug)]
pub struct Creature {
    pub creature_state: CreatureState,
    pub position: Vec2,
    pub metadata: Metadata,
    pub sprite_state: SpriteState,
    pub sprite_state_duration: usize,
}

impl Creature {
    /**
     * Initializes a new creature from metadata with a (uniformly) random state
     * and a random position.
     *
     * NOTE: This position is not bound!
     */
    pub fn from_metadata<R: Rng + ?Sized>(metadata: Metadata, rng: &mut R) -> Self {
        let state: CreatureState = CreatureState::random(rng);
        let position: Vec2 = (rng.random::<f64>(), rng.random::<f64>()).into();
        Creature {
            creature_state: state,
            position,
            metadata,
            sprite_state: SpriteState::Idle,
            sprite_state_duration: 0,
        }
    }

    /**
     * Computes the next state (randomly) for the creature.
     * DOES NOT REPOSITION THE CREATURE. THE COLLIDABLE DOES NOT CHANGE.
     * THE SPRITE STATE DOES NOT CHANGE.
     */
    pub fn into_next_state<R: Rng + ?Sized>(self, rng: &mut R) -> Self {
        let next_state = self.creature_state.next(rng);
        Creature {
            creature_state: next_state,
            position: self.position,
            metadata: self.metadata,
            sprite_state: self.sprite_state,
            sprite_state_duration: self.sprite_state_duration,
        }
    }

    /**
     * Has the creature take a step in the direction.
     * Changes the sprite.
     */
    pub fn step(self, step: Step) -> Self {
        let new_sprite_state = match SpriteState::from_delta(&step.delta) {
            Some(s) => s,
            None if self.creature_state == CreatureState::Sleep => SpriteState::Sleep,
            None => SpriteState::Idle,
        };
        let new_sprite_state_duration = if new_sprite_state == self.sprite_state {
            self.sprite_state_duration + 1
        } else {
            0
        };
        let new_position = step.collapse().position;

        Creature {
            creature_state: self.creature_state,
            position: new_position,
            metadata: self.metadata,
            sprite_state: new_sprite_state,
            sprite_state_duration: new_sprite_state_duration,
        }
    }

    /**
     * Set position field WITHOUT CHANGING ANYTHING ELSE
     */
    pub fn set_position(self, position: Vec2) -> Self {
        Creature {
            creature_state: self.creature_state,
            position,
            metadata: self.metadata,
            sprite_state: self.sprite_state,
            sprite_state_duration: self.sprite_state_duration,
        }
    }

    /**
     * Calculates the next step given the creature's position
     * and a center of mass to trend toward.
     */
    pub fn get_next_step<R: Rng + ?Sized>(&self, center_of_mass: &Vec2, rng: &mut R) -> Step {
        match self.creature_state {
            CreatureState::Follow => {
                let delta = center_of_mass - &self.position;
                let jitter = &(delta.norm() * JITTER_STRENGTH) * &Vec2::random(rng);
                let s = Step::new(
                    self.as_collidable(),
                    (delta + jitter).with_norm(self.metadata.step_size),
                );
                s
            }
            CreatureState::Flee => {
                let delta = &self.position - center_of_mass;
                let jitter = &(delta.norm() * JITTER_STRENGTH) * &Vec2::random(rng);
                let s = Step::new(
                    self.as_collidable(),
                    (delta + jitter).with_norm(self.metadata.step_size),
                );
                s
            }
            _ => Step::new(self.as_collidable(), Vec2::zero()),
        }
    }

    pub fn as_collidable(&self) -> Collidable {
        Collidable::new(self.position.clone(), self.metadata.radius)
    }

    pub fn radius(&self) -> f64 {
        self.metadata.radius.clone()
    }

    pub fn get_sprite(&self) -> Option<Vec<u8>> {
        match &self.metadata.sprite_sheet {
            Some(sprite_sheet) => {
                Some(sprite_sheet.get_sprite(&self.sprite_state, self.sprite_state_duration))
            }
            None => None,
        }
    }
}

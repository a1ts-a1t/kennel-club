use std::path::Path;

pub use metadata::Metadata;
use rand::Rng;
pub use state::State;

use crate::math::Vec2;
use crate::physics::{Body, Collider};
use crate::{Sprite, sprite};

mod metadata;
mod state;

#[derive(Debug)]
pub struct Creature {
    pub id: String,
    pub display_name: String,
    pub step_size: f64,
    pub radius: f64,
    pub url: String,
    pub creature_state: State,
    pub position: Vec2,
    pub sprite_state: sprite::State,
    pub sprite_frame: usize,
    pub sprite_sheet: sprite::Sheet,
}

#[cfg(test)]
impl From<Metadata> for Creature {
    fn from(metadata: Metadata) -> Self {
        let sprite_sheet = sprite::Sheet::new();
        Creature {
            id: metadata.id,
            display_name: metadata.display_name,
            radius: metadata.radius,
            step_size: metadata.step_size,
            url: metadata.url,
            creature_state: metadata.initial_state,
            position: Vec2::zero(),
            sprite_state: sprite::State::Idle,
            sprite_frame: 0,
            sprite_sheet,
        }
    }
}

impl Creature {
    pub fn load(metadata: Metadata, data_dir: &Path) -> Self {
        let sprite_sheet = metadata.sprite_loader.load(&data_dir.join(&metadata.id));
        Creature {
            id: metadata.id,
            display_name: metadata.display_name,
            radius: metadata.radius,
            step_size: metadata.step_size,
            url: metadata.url,
            creature_state: metadata.initial_state,
            position: Vec2::zero(),
            sprite_state: sprite::State::Idle,
            sprite_frame: 0,
            sprite_sheet,
        }
    }

    /// Computes the next state (randomly) for the creature.
    /// DOES NOT REPOSITION THE CREATURE. THE COLLIDABLE DOES NOT CHANGE.
    /// THE SPRITE STATE DOES NOT CHANGE.
    pub fn with_next_state<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let next_state = self.creature_state.next(rng);
        Creature {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            radius: self.radius,
            step_size: self.step_size,
            url: self.url.clone(),
            creature_state: next_state,
            position: self.position,
            sprite_state: self.sprite_state,
            sprite_frame: self.sprite_frame,
            sprite_sheet: self.sprite_sheet.clone(),
        }
    }

    pub fn resolve_body(self, body: Body) -> Self {
        let new_sprite_state = match sprite::State::from_delta(body.velocity()) {
            Some(s) => s,
            None if self.creature_state == State::Sleep => sprite::State::Sleep,
            None => sprite::State::Idle,
        };

        let frame_count = self.sprite_sheet.get_frame_count(self.sprite_state);
        let new_sprite_frame = if new_sprite_state != self.sprite_state || frame_count == 0 {
            0
        } else {
            (self.sprite_frame + 1) % frame_count
        };

        let new_position = body.collider().centroid().unwrap();
        Creature {
            id: self.id,
            display_name: self.display_name,
            radius: self.radius,
            step_size: self.step_size,
            creature_state: self.creature_state,
            url: self.url,
            position: new_position,
            sprite_state: new_sprite_state,
            sprite_frame: new_sprite_frame,
            sprite_sheet: self.sprite_sheet,
        }
    }

    /// Set position field WITHOUT CHANGING ANYTHING ELSE
    pub fn set_position(self, position: Vec2) -> Self {
        Creature {
            id: self.id,
            display_name: self.display_name,
            radius: self.radius,
            step_size: self.step_size,
            url: self.url,
            creature_state: self.creature_state,
            position,
            sprite_state: self.sprite_state,
            sprite_frame: self.sprite_frame,
            sprite_sheet: self.sprite_sheet,
        }
    }

    pub fn as_collider(&self) -> Collider {
        Collider::new_circle(self.position, self.radius)
    }

    pub fn as_body(&self, center_of_mass: Vec2) -> Body {
        let velocity = match self.creature_state {
            State::Follow => (center_of_mass - self.position).with_norm(self.step_size),
            State::Flee => (self.position - center_of_mass).with_norm(self.step_size),
            _ => Vec2::zero(),
        };

        let collider = Collider::new_circle(self.position, self.radius);

        Body::new(collider)
            .with_velocity(velocity)
            .with_mass(self.radius)
    }

    pub fn sprite(&self) -> &Sprite {
        self.sprite_sheet
            .get_sprite(self.sprite_state, self.sprite_frame)
    }
}

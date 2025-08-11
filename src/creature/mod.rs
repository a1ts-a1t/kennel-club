pub use metadatum::Metadatum;
use rand::Rng;
use state::{State, StateType};

use crate::{
    collidable::Collidable,
    vec2::{Position, Vec2},
};

mod metadatum;
mod state;

#[derive(Clone)]
pub struct Creature {
    pub state: State,
    // sprite: String, // some combination of sprite sheet metadata and state duration
    pub collidable: Collidable,
    pub metadatum: Metadatum,
}

impl Creature {
    /**
     * Initializes a new creature from metadatum with a (uniformly) random state
     * and a random position.
     *
     * NOTE: This position is not bound!
     */
    pub fn from_metadatum<R: Rng + ?Sized>(metadatum: Metadatum, rng: &mut R) -> Self {
        let state: State = StateType::random(rng).into();
        let position: Position = (rng.random::<f64>(), rng.random::<f64>()).into();
        let collidable = Collidable::new(position, metadatum.radius);
        Creature {
            state,
            collidable,
            metadatum,
        }
    }

    /**
     * Computes the next state (randomly) for the creature and consumes it.
     * DOES NOT REPOSITION THE CREATURE. THE COLLIDABLE DOES NOT CHANGE.
     */
    pub fn next_state(self) -> Self {
        todo!();
    }

    /**
     * Moves the creature to the new position and consumes it.
     */
    pub fn reposition(self, new_position: Vec2) -> Self {
        let new_collidable = Collidable::new(new_position, self.metadatum.radius);
        Creature {
            state: self.state,
            collidable: new_collidable,
            metadatum: self.metadatum,
        }
    }

    pub fn position(&self) -> Position {
        self.collidable.position.clone()
    }
}

pub use metadatum::Metadatum;
use rand::Rng;
pub use state::{State, StateType};

use crate::{math::Vec2, physics::Collidable};

mod metadatum;
mod state;

#[derive(Clone, Debug)]
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
        let position: Vec2 = (rng.random::<f64>(), rng.random::<f64>()).into();
        let collidable = Collidable::new(position, metadatum.radius);
        Creature {
            state,
            collidable,
            metadatum,
        }
    }

    /**
     * Computes the next state (randomly) for the creature.
     * DOES NOT REPOSITION THE CREATURE. THE COLLIDABLE DOES NOT CHANGE.
     */
    pub fn next_state<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let next_state = self.state.next(rng);
        Creature {
            state: next_state,
            collidable: self.collidable.clone(),
            metadatum: self.metadatum.clone(),
        }
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

    pub fn position(&self) -> Vec2 {
        self.collidable.position.clone()
    }
}

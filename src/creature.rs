use rand::{
    Rng,
    distr::{Distribution, StandardUniform, weighted::WeightedIndex},
};
use serde::Serialize;

use crate::math::Vec2;

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum CreatureState {
    Idle(u8),
    Sleep(u8),
    Flee(u8),
    Follow(u8),
}

impl Distribution<CreatureState> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CreatureState {
        let r = rng.random_range(0..=4);
        match r {
            0 => CreatureState::Idle(0),
            1 => CreatureState::Sleep(0),
            2 => CreatureState::Flee(0),
            _ => CreatureState::Follow(0),
        }
    }
}

impl CreatureState {
    pub fn random() -> CreatureState {
        rand::random()
    }

    pub fn next<R: Rng + ?Sized>(&self, rng: &mut R) -> CreatureState {
        let weights = self.weights();
        let distr = WeightedIndex::new(weights).unwrap();
        let index = rng.sample(distr);

        match self {
            CreatureState::Idle(duration) if index == 0 => CreatureState::Idle(duration + 1),
            CreatureState::Sleep(duration) if index == 1 => CreatureState::Sleep(duration + 1),
            CreatureState::Flee(duration) if index == 1 => CreatureState::Flee(duration + 1),
            CreatureState::Follow(duration) if index == 1 => CreatureState::Follow(duration + 1),
            _ if index == 0 => CreatureState::Idle(0),
            _ if index == 1 => CreatureState::Sleep(0),
            _ if index == 2 => CreatureState::Flee(0),
            _ => CreatureState::Follow(0),
        }
    }

    #[rustfmt::skip]
    fn weights(&self) -> [u8; 4] {
        match self {
            CreatureState::Idle(_) =>   [75, 15,  5,  5],
            CreatureState::Sleep(_) =>  [10, 90,  0,  0],
            CreatureState::Flee(_) =>   [10,  0, 75, 15],
            CreatureState::Follow(_) => [10,  0, 15, 75],
        }
    }
}

#[derive(Serialize, Clone)]
pub struct CreatureMetadata {
    display_name: String,
    url: String,
    // TODO: sprite data?
}

#[derive(Serialize, Clone)]
pub struct Creature {
    pub metadata: CreatureMetadata,
    pub position: Vec2,
    pub state: CreatureState,
}

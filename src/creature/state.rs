use rand::{Rng, distr::weighted::WeightedIndex};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum State {
    Idle,
    Sleep,
    Follow,
    Flee,
}

impl State {
    /**
     * Given a current state type, generate the next one
     * based on some transition matrix.
     */
    pub fn next<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let weights = self.weights();
        let distr = WeightedIndex::new(weights).unwrap();
        let index = rng.sample(distr);

        match index {
            0 => State::Idle,
            1 => State::Sleep,
            2 => State::Flee,
            _ => State::Follow,
        }
    }

    /**
     * TODO: can this be configured by users?
     */
    #[rustfmt::skip]
    fn weights(&self) -> [u8; 4] {
        match self {
            State::Idle =>   [75, 15,  5,  5],
            State::Sleep =>  [10, 90,  0,  0],
            State::Flee =>   [10,  0, 75, 15],
            State::Follow => [10,  0, 15, 75],
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

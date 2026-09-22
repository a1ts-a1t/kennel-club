use rand::{Rng, distr::weighted::WeightedIndex};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum State {
    #[default]
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
            State::Idle =>   [40, 20, 20, 20],
            State::Sleep =>  [20, 80,  0,  0],
            State::Flee =>   [10,  0, 75, 15],
            State::Follow => [10,  0, 15, 75],
        }
    }
}

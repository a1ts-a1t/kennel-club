use rand::{Rng, distr::weighted::WeightedIndex};

#[derive(Clone, PartialEq, Debug)]
pub enum StateType {
    Idle,
    Sleep,
    Follow,
    Flee,
}

impl StateType {
    /**
     * Get a random state, uniformly.
     */
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.random_range(0..=3) {
            0 => StateType::Idle,
            1 => StateType::Sleep,
            2 => StateType::Follow,
            _ => StateType::Flee,
        }
    }

    /**
     * Given a current state type, generate the next one
     * based on some transition matrix.
     */
    pub fn next<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let weights = self.weights();
        let distr = WeightedIndex::new(weights).unwrap();
        let index = rng.sample(distr);

        match index {
            0 => StateType::Idle,
            1 => StateType::Sleep,
            2 => StateType::Flee,
            _ => StateType::Follow,
        }
    }

    /**
     * TODO: can this be configured by users?
     */
    #[rustfmt::skip]
    fn weights(&self) -> [u8; 4] {
        match self {
            StateType::Idle =>   [75, 15,  5,  5],
            StateType::Sleep =>  [10, 90,  0,  0],
            StateType::Flee =>   [10,  0, 75, 15],
            StateType::Follow => [10,  0, 15, 75],
        }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    pub state_type: StateType,
    pub duration: u8,
}

impl State {
    /**
     * Given a current state, compute the next one
     */
    pub fn next<R: Rng + ?Sized>(&self, rng: &mut R) -> Self {
        let next_state_type = self.state_type.next(rng);
        let next_duration = if next_state_type == self.state_type {
            self.duration + 1
        } else {
            0
        };
        State {
            state_type: next_state_type,
            duration: next_duration,
        }
    }
}

impl From<StateType> for State {
    fn from(value: StateType) -> Self {
        State {
            state_type: value,
            duration: 0,
        }
    }
}

use rand::{Rng, distr::weighted::WeightedIndex};

#[derive(Clone, PartialEq, Debug)]
pub enum CreatureState {
    Idle,
    Sleep,
    Follow,
    Flee,
}

impl CreatureState {
    /**
     * Get a random state, uniformly.
     */
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.random_range(0..=3) {
            0 => CreatureState::Idle,
            1 => CreatureState::Sleep,
            2 => CreatureState::Follow,
            _ => CreatureState::Flee,
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
            0 => CreatureState::Idle,
            1 => CreatureState::Sleep,
            2 => CreatureState::Flee,
            _ => CreatureState::Follow,
        }
    }

    /**
     * TODO: can this be configured by users?
     */
    #[rustfmt::skip]
    fn weights(&self) -> [u8; 4] {
        match self {
            CreatureState::Idle =>   [75, 15,  5,  5],
            CreatureState::Sleep =>  [10, 90,  0,  0],
            CreatureState::Flee =>   [10,  0, 75, 15],
            CreatureState::Follow => [10,  0, 15, 75],
        }
    }
}

use rand::Rng;

#[derive(Clone)]
pub enum StateType {
    Idle,
    Sleep,
    Follow,
    Flee,
}

impl StateType {
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
    pub fn next(self) -> Self {
        todo!();
    }
}

#[derive(Clone)]
pub struct State {
    state_type: StateType,
    duration: u8,
}

impl State {
    /**
     * Given a current state, compute the next one and consume this state
     */
    pub fn next(self) -> Self {
        todo!();
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

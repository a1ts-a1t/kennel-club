use crate::{collidable::Collidable, vec2::Delta};

#[derive(Clone)]
pub struct Step {
    pub collidable: Collidable,
    delta: Delta,
}

pub enum ResolutionResult<T> {
    Ok,
    ResolvedTo(T),
    Err,
}

impl Step {
    pub fn new(collidable: Collidable, delta: Delta) -> Self {
        Self { collidable, delta }
    }

    /**
     * Collapses the step and adds the delta to the collidable's position.
     */
    pub fn collapse(self) -> Collidable {
        Collidable { position: self.collidable.position + self.delta, radius: self.collidable.radius }
    }

    pub fn resolve_to_bounds(&self, x_bounds: &(f64, f64), y_bounds: &(f64, f64)) -> ResolutionResult<Self> {
        todo!();
    }

    pub fn resolve_steps(step1: &Self, step2: &Self) -> ResolutionResult<(Self, Self)> {
        todo!();
    }
}

use serde::Deserialize;

use crate::{creature::state::State, sprite};

#[cfg(test)]
use rand::{Rng, distr::Alphabetic};

/**
 * Metadata that's loaded in from JSON
 * Do not directly construct.
 */

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub id: String,
    pub step_size: f64,
    pub radius: f64,
    #[serde(alias = "sprites")]
    pub sprite_loader: sprite::Loader,
    #[serde(default)]
    pub initial_state: State,
}

impl Metadata {
    #[cfg(test)]
    pub fn mock(radius: f64) -> Self {
        let id = rand::rng()
            .sample_iter(&Alphabetic)
            .take(5)
            .map(char::from)
            .collect();

        Metadata {
            id,
            step_size: 0.0,
            radius,
            sprite_loader: sprite::Loader::new(),
            initial_state: State::Idle,
        }
    }
}

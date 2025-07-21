use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct KennelMetadata {
    pub width: f64,
    pub height: f64,
    pub creature_radius: f64, // half the side length of a creature collision box
    pub creature_count: usize,
    pub step_size: f64,
}

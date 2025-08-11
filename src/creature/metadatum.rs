#[derive(Clone, Debug)]
pub struct Metadatum {
    pub id: String,
    pub step_size: f64,
    pub radius: f64,
    // information that stays static throughout the lifetime of the creature
}

impl Metadatum {
    pub fn new(id: String, step_size: f64, radius: f64) -> Self {
        Self {
            id,
            step_size,
            radius,
        }
    }
}

use crate::sprite::SpriteSheet;

#[derive(Debug)]
pub struct Metadata {
    pub id: String,
    pub step_size: f64,
    pub radius: f64,
    pub sprite_sheet: Option<SpriteSheet>,
    // information that stays static throughout the lifetime of the creature
}

impl Metadata {
    pub fn new(id: String, step_size: f64, radius: f64, sprite_sheet: Option<SpriteSheet>) -> Self {
        Self {
            id,
            step_size,
            radius,
            sprite_sheet,
        }
    }
}

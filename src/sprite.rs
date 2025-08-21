use std::{cmp::Ordering, f64::consts::PI, fmt, ops::Range};

use crate::math::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub enum SpriteState {
    Idle,
    Sleep,
    East,
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
}

static SPRITE_STATE_RANGES: [Range<f64>; 7] = [
    (-7.0 * PI / 8.0)..(-5.0 * PI / 8.0),
    (-5.0 * PI / 8.0)..(-3.0 * PI / 8.0),
    (-3.0 * PI / 8.0)..(-1.0 * PI / 8.0),
    (-1.0 * PI / 8.0)..(1.0 * PI / 8.0),
    (1.0 * PI / 8.0)..(3.0 * PI / 8.0),
    (3.0 * PI / 8.0)..(5.0 * PI / 8.0),
    (5.0 * PI / 8.0)..(7.0 * PI / 8.0),
];

impl SpriteState {
    pub fn from_delta(delta: &Vec2) -> Option<Self> {
        if delta.x == 0.0 && delta.y == 0.0 {
            return None;
        }

        let theta = f64::atan2(delta.y, delta.x);

        if SPRITE_STATE_RANGES[0].contains(&theta) {
            return Some(SpriteState::Southwest);
        }

        if SPRITE_STATE_RANGES[1].contains(&theta) {
            return Some(SpriteState::South);
        }

        if SPRITE_STATE_RANGES[2].contains(&theta) {
            return Some(SpriteState::Southeast);
        }

        if SPRITE_STATE_RANGES[3].contains(&theta) {
            return Some(SpriteState::East);
        }

        if SPRITE_STATE_RANGES[4].contains(&theta) {
            return Some(SpriteState::Northeast);
        }

        if SPRITE_STATE_RANGES[5].contains(&theta) {
            return Some(SpriteState::North);
        }

        if SPRITE_STATE_RANGES[6].contains(&theta) {
            return Some(SpriteState::Northwest);
        }

        Some(SpriteState::West)
    }
}

pub struct SpriteSheet {
    idle: Vec<Vec<u8>>,
    sleep: Vec<Vec<u8>>,
    east: Vec<Vec<u8>>,
    northeast: Vec<Vec<u8>>,
    north: Vec<Vec<u8>>,
    northwest: Vec<Vec<u8>>,
    west: Vec<Vec<u8>>,
    southwest: Vec<Vec<u8>>,
    south: Vec<Vec<u8>>,
    southeast: Vec<Vec<u8>>,
}

impl SpriteSheet {
    pub fn load() -> Self {
        todo!();
    }

    pub fn get_sprite(&self, sprite_state: &SpriteState, frame: usize) -> Vec<u8> {
        let frame_idx = match sprite_state {
            SpriteState::Idle => frame % self.idle.len(),
            SpriteState::Sleep => frame % self.sleep.len(),
            SpriteState::East => frame % self.east.len(),
            SpriteState::Northeast => frame % self.northeast.len(),
            SpriteState::North => frame % self.north.len(),
            SpriteState::Northwest => frame % self.northwest.len(),
            SpriteState::West => frame % self.west.len(),
            SpriteState::Southwest => frame % self.southwest.len(),
            SpriteState::South => frame % self.south.len(),
            SpriteState::Southeast => frame % self.southeast.len(),
        };

        match sprite_state {
            SpriteState::Idle => self.idle.get(frame_idx).unwrap().to_vec(),
            SpriteState::Sleep => self.sleep.get(frame_idx).unwrap().to_vec(),
            SpriteState::East => self.east.get(frame_idx).unwrap().to_vec(),
            SpriteState::Northeast => self.northeast.get(frame_idx).unwrap().to_vec(),
            SpriteState::North => self.north.get(frame_idx).unwrap().to_vec(),
            SpriteState::Northwest => self.northwest.get(frame_idx).unwrap().to_vec(),
            SpriteState::West => self.west.get(frame_idx).unwrap().to_vec(),
            SpriteState::Southwest => self.southwest.get(frame_idx).unwrap().to_vec(),
            SpriteState::South => self.south.get(frame_idx).unwrap().to_vec(),
            SpriteState::Southeast => self.southeast.get(frame_idx).unwrap().to_vec(),
        }
    }
}

impl fmt::Debug for SpriteSheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Default for SpriteSheet {
    fn default() -> Self {
        SpriteSheet {
            idle: vec![],
            sleep: vec![],
            east: vec![],
            northeast: vec![],
            north: vec![],
            northwest: vec![],
            west: vec![],
            southwest: vec![],
            south: vec![],
            southeast: vec![],
        }
    }
}

use std::{f64::consts::PI, fmt, fs, ops::Range, path::{Path, PathBuf}};

use serde::{de, Deserialize, Deserializer};

use crate::math::Vec2;

static SPRITE_ROOT: &str = "../data";

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

struct SpriteSheetLoader {
    id: String,
    idle_file_names: Vec<String>,
    sleep_file_names: Vec<String>,
    east_file_names: Vec<String>,
    northeast_file_names: Vec<String>,
    north_file_names: Vec<String>,
    northwest_file_names: Vec<String>,
    west_file_names: Vec<String>,
    southwest_file_names: Vec<String>,
    south_file_names: Vec<String>,
    southeast_file_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpriteSheet {
    #[serde(deserialize_with = "from_paths")]
    idle: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    sleep: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    east: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    northeast: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    north: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    northwest: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    west: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    southwest: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    south: Vec<Vec<u8>>,
    #[serde(deserialize_with = "from_paths")]
    southeast: Vec<Vec<u8>>,
}

fn from_paths<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>
{
    let path_strs: Vec<String> = Deserialize::deserialize(deserializer)?;
    let sprite_data: Result<Vec<_>, _>= path_strs.into_iter()
        .map(|path_str| PathBuf::from(path_str))
        .map(|path| PathBuf::from(SPRITE_ROOT).join(path))
        .map(|path| fs::read(path))
        .collect();

    match sprite_data {
        Ok(d) => Ok(d),
        Err(e) => Err(de::Error::custom(e)),
    }
}

impl SpriteSheet {
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

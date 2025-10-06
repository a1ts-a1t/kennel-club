use std::{f64::consts::PI, ops::Range};

use crate::math::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
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

#[rustfmt::skip]
static SPRITE_STATE_RANGES: [Range<f64>; 7] = [
    (-7.0 * PI / 8.0)..(-5.0 * PI / 8.0),
    (-5.0 * PI / 8.0)..(-3.0 * PI / 8.0),
    (-3.0 * PI / 8.0)..(      -PI / 8.0),
    (      -PI / 8.0)..( 1.0 * PI / 8.0),
    ( 1.0 * PI / 8.0)..( 3.0 * PI / 8.0),
    ( 3.0 * PI / 8.0)..( 5.0 * PI / 8.0),
    ( 5.0 * PI / 8.0)..( 7.0 * PI / 8.0),
];

impl State {
    pub fn from_delta(delta: &Vec2) -> Option<Self> {
        if delta.x == 0.0 && delta.y == 0.0 {
            return None;
        }

        let theta = f64::atan2(delta.y, delta.x);

        if SPRITE_STATE_RANGES[0].contains(&theta) {
            return Some(State::Southwest);
        }

        if SPRITE_STATE_RANGES[1].contains(&theta) {
            return Some(State::South);
        }

        if SPRITE_STATE_RANGES[2].contains(&theta) {
            return Some(State::Southeast);
        }

        if SPRITE_STATE_RANGES[3].contains(&theta) {
            return Some(State::East);
        }

        if SPRITE_STATE_RANGES[4].contains(&theta) {
            return Some(State::Northeast);
        }

        if SPRITE_STATE_RANGES[5].contains(&theta) {
            return Some(State::North);
        }

        if SPRITE_STATE_RANGES[6].contains(&theta) {
            return Some(State::Northwest);
        }

        Some(State::West)
    }
}

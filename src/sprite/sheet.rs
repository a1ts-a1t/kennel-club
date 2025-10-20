use std::path::PathBuf;

use crate::sprite::base::Sprite;

use super::state::State;

#[derive(Debug, Default, Clone)]
pub struct Sheet {
    idle: Vec<Sprite>,
    sleep: Vec<Sprite>,
    east: Vec<Sprite>,
    northeast: Vec<Sprite>,
    north: Vec<Sprite>,
    northwest: Vec<Sprite>,
    west: Vec<Sprite>,
    southwest: Vec<Sprite>,
    south: Vec<Sprite>,
    southeast: Vec<Sprite>,
}

impl Sheet {
    pub(crate) fn new() -> Self {
        Sheet {
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

    pub(crate) fn push_idle(&mut self, path: PathBuf) {
        self.idle.push(Sprite::load(&path));
    }

    pub(crate) fn push_sleep(&mut self, path: PathBuf) {
        self.sleep.push(Sprite::load(&path));
    }

    pub(crate) fn push_east(&mut self, path: PathBuf) {
        self.east.push(Sprite::load(&path));
    }

    pub(crate) fn push_northeast(&mut self, path: PathBuf) {
        self.northeast.push(Sprite::load(&path));
    }

    pub(crate) fn push_north(&mut self, path: PathBuf) {
        self.north.push(Sprite::load(&path));
    }

    pub(crate) fn push_northwest(&mut self, path: PathBuf) {
        self.northwest.push(Sprite::load(&path));
    }

    pub(crate) fn push_west(&mut self, path: PathBuf) {
        self.west.push(Sprite::load(&path));
    }

    pub(crate) fn push_southwest(&mut self, path: PathBuf) {
        self.southwest.push(Sprite::load(&path));
    }

    pub(crate) fn push_south(&mut self, path: PathBuf) {
        self.south.push(Sprite::load(&path));
    }

    pub(crate) fn push_southeast(&mut self, path: PathBuf) {
        self.southeast.push(Sprite::load(&path));
    }

    pub fn get_sprite(&self, sprite_state: &State, frame: usize) -> &Sprite {
        let frame_idx = match sprite_state {
            State::Idle => frame % self.idle.len(),
            State::Sleep => frame % self.sleep.len(),
            State::East => frame % self.east.len(),
            State::Northeast => frame % self.northeast.len(),
            State::North => frame % self.north.len(),
            State::Northwest => frame % self.northwest.len(),
            State::West => frame % self.west.len(),
            State::Southwest => frame % self.southwest.len(),
            State::South => frame % self.south.len(),
            State::Southeast => frame % self.southeast.len(),
        };

        match sprite_state {
            State::Idle => self.idle.get(frame_idx).unwrap(),
            State::Sleep => self.sleep.get(frame_idx).unwrap(),
            State::East => self.east.get(frame_idx).unwrap(),
            State::Northeast => self.northeast.get(frame_idx).unwrap(),
            State::North => self.north.get(frame_idx).unwrap(),
            State::Northwest => self.northwest.get(frame_idx).unwrap(),
            State::West => self.west.get(frame_idx).unwrap(),
            State::Southwest => self.southwest.get(frame_idx).unwrap(),
            State::South => self.south.get(frame_idx).unwrap(),
            State::Southeast => self.southeast.get(frame_idx).unwrap(),
        }
    }
}

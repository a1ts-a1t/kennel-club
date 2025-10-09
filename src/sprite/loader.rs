use std::path::PathBuf;

use serde::Deserialize;

use crate::sprite::Sheet;

#[derive(Debug, Deserialize)]
pub struct Loader {
    idle: Vec<String>,
    sleep: Vec<String>,
    east: Vec<String>,
    northeast: Vec<String>,
    north: Vec<String>,
    northwest: Vec<String>,
    west: Vec<String>,
    southwest: Vec<String>,
    south: Vec<String>,
    southeast: Vec<String>,
}

impl Loader {
    #[cfg(test)]
    pub fn new() -> Self {
        Loader {
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

    pub fn load(self, path_prefix: &PathBuf) -> Sheet {
        let mut sheet = Sheet::new();

        for path in self.idle.into_iter() {
            sheet.push_idle(path_prefix.join(path));
        }

        for path in self.sleep.into_iter() {
            sheet.push_sleep(path_prefix.join(path));
        }

        for path in self.east.into_iter() {
            sheet.push_east(path_prefix.join(path));
        }

        for path in self.northeast.into_iter() {
            sheet.push_northeast(path_prefix.join(path));
        }

        for path in self.north.into_iter() {
            sheet.push_north(path_prefix.join(path));
        }

        for path in self.northwest.into_iter() {
            sheet.push_northwest(path_prefix.join(path));
        }

        for path in self.west.into_iter() {
            sheet.push_west(path_prefix.join(path));
        }

        for path in self.southwest.into_iter() {
            sheet.push_southwest(path_prefix.join(path));
        }

        for path in self.south.into_iter() {
            sheet.push_south(path_prefix.join(path));
        }

        for path in self.southeast.into_iter() {
            sheet.push_southeast(path_prefix.join(path));
        }

        sheet
    }
}

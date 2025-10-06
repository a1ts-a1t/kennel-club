use serde::Deserialize;

use crate::{env::data_dir, sprite::Sheet};

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

    pub fn load(self, path_prefix: &str) -> Sheet {
        let mut sheet = Sheet::new();
        let base_path = data_dir().join(path_prefix);

        for path in self.idle.into_iter() {
            sheet.push_idle(base_path.join(path));
        }

        for path in self.sleep.into_iter() {
            sheet.push_sleep(base_path.join(path));
        }

        for path in self.east.into_iter() {
            sheet.push_east(base_path.join(path));
        }

        for path in self.northeast.into_iter() {
            sheet.push_northeast(base_path.join(path));
        }

        for path in self.north.into_iter() {
            sheet.push_north(base_path.join(path));
        }

        for path in self.northwest.into_iter() {
            sheet.push_northwest(base_path.join(path));
        }

        for path in self.west.into_iter() {
            sheet.push_west(base_path.join(path));
        }

        for path in self.southwest.into_iter() {
            sheet.push_southwest(base_path.join(path));
        }

        for path in self.south.into_iter() {
            sheet.push_south(base_path.join(path));
        }

        for path in self.southeast.into_iter() {
            sheet.push_southeast(base_path.join(path));
        }

        sheet
    }
}

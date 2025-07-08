use std::ops::IndexMut;

use itertools::Itertools;

use crate::math::Vec2;

use super::metadata::KennelMetadata;

pub struct CollisionLattice {
    width: usize,
    height: usize,
    lattice_distance: f64, // the distance between points on a lattice (horizontally or vertically)
    grid: Box<[Box<[Option<Vec<Vec2>>]>]>,
    pub count: usize,
}

impl CollisionLattice {
    pub fn add(&mut self, position: Vec2) {
        self.count += 1;
        let x_idx = (position.x / self.lattice_distance).floor() as usize;
        let y_idx = (position.y / self.lattice_distance).floor() as usize;

        let ele = self.grid.index_mut(x_idx).index_mut(y_idx);
        match ele {
            Some(v) => v.push(position),
            None => *ele = Some(vec![position]),
        };
    }

    pub fn get_collision_candidates(&self, position: &Vec2) -> Vec<Vec2> {
        let mut candidates: Vec<Vec2> = vec![];

        let x_idx = (position.x / self.lattice_distance).floor() as usize;
        let y_idx = (position.y / self.lattice_distance).floor() as usize;

        if let Some(ref v) = self.grid[x_idx][y_idx] {
            candidates.extend_from_slice(v);
        }

        // fetch surrounding tiles if they are in bounds.
        // i didn't want to incur the cost of extra abstractions
        // so here's a bunch of if statements. eat your heart out.
        let x_lower = x_idx > 0;
        let x_higher = x_idx < self.width - 1;
        let y_lower = y_idx > 0;
        let y_higher = y_idx < self.height - 1;

        if (x_lower && y_lower)
            && let Some(ref v) = self.grid[x_idx - 1][y_idx - 1]
        {
            candidates.extend_from_slice(v);
        }

        if (x_lower) && let Some(ref v) = self.grid[x_idx - 1][y_idx] {
            candidates.extend_from_slice(v);
        }

        if (x_lower && y_higher)
            && let Some(ref v) = self.grid[x_idx - 1][y_idx + 1]
        {
            candidates.extend_from_slice(v);
        }

        if (y_lower) && let Some(ref v) = self.grid[x_idx][y_idx - 1] {
            candidates.extend_from_slice(v);
        }

        if (y_higher) && let Some(ref v) = self.grid[x_idx][y_idx + 1] {
            candidates.extend_from_slice(v);
        }

        if (x_higher && y_lower)
            && let Some(ref v) = self.grid[x_idx + 1][y_idx - 1]
        {
            candidates.extend_from_slice(v);
        }

        if (x_higher) && let Some(ref v) = self.grid[x_idx + 1][y_idx] {
            candidates.extend_from_slice(v);
        }

        if (x_higher && y_higher)
            && let Some(ref v) = self.grid[x_idx + 1][y_idx + 1]
        {
            candidates.extend_from_slice(v);
        }

        candidates
    }

    pub fn into_vec(self) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = vec![];
        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            if let Some(ref positions) = self.grid[x][y] {
                v.extend_from_slice(positions);
            }
        }
        v
    }
}

impl From<&KennelMetadata> for CollisionLattice {
    fn from(metadata: &KennelMetadata) -> Self {
        let hitbox_size = 2f64 * metadata.creature_radius;
        let width = (metadata.width / hitbox_size).floor() as usize;
        let height = (metadata.height / hitbox_size).floor() as usize;
        let grid = vec![vec![None; height].into_boxed_slice(); width].into_boxed_slice();
        CollisionLattice {
            width,
            height,
            lattice_distance: hitbox_size,
            grid,
            count: 0,
        }
    }
}

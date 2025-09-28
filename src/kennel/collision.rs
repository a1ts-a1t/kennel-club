use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::physics::Step;

#[derive(PartialEq)]
enum ArenaCollision {
    Bound(usize, f64),
    Steps((usize, usize), f64),
}

impl ArenaCollision {
    fn new_bound_collision(idx: usize, time: f64) -> Self {
        Self::Bound(idx, time)
    }

    fn new_steps_collision(indices: (usize, usize), time: f64) -> Self {
        Self::Steps(indices, time)
    }

    fn time(&self) -> f64 {
        match self {
            ArenaCollision::Bound(_, time) => *time,
            ArenaCollision::Steps(_, time) => *time,
        }
    }
}

impl Eq for ArenaCollision {}

impl PartialOrd for ArenaCollision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArenaCollision {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time().total_cmp(&other.time()).reverse()
    }
}

pub struct Arena {
    steps: Vec<Step>,
    heap: BinaryHeap<ArenaCollision>,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            steps: Vec::new(),
            heap: BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, new_step: Step) {
        let new_idx = self.steps.len();

        // add in bound collisions
        if let Some(time) = new_step.unit_bound_collision_time() {
            let collision = ArenaCollision::new_bound_collision(new_idx, time);
            self.heap.push(collision);
        }

        // add in step collisions
        self.steps
            .iter()
            .filter_map(|step| Step::steps_collision_time(step, &new_step))
            .enumerate()
            .map(|(idx, time)| ArenaCollision::new_steps_collision((idx, new_idx), time))
            .for_each(|collision| self.heap.push(collision));

        self.steps.push(new_step);
    }

    pub fn into_vec(self) -> Vec<Step> {
        let mut visited_indices = HashSet::<usize>::new();
        let mut vec: Vec<Step> = vec![Step::default(); self.steps.len()];

        // resolve all collisions
        for collision in self.heap.into_iter() {
            if collision.time() >= 1.0 {
                break;
            }

            // if some step in this collision already got resolved, skip it
            match collision {
                ArenaCollision::Bound(idx, _) if visited_indices.contains(&idx) => continue,
                ArenaCollision::Steps((idx, _), _) if visited_indices.contains(&idx) => continue,
                ArenaCollision::Steps((_, idx), _) if visited_indices.contains(&idx) => continue,
                _ => (),
            }

            // add the collision into the final vec
            let mut add_collision = |idx: usize, time: f64| {
                let lerped_step = self
                    .steps
                    .get(idx)
                    .expect("Unable to retrieve step from collision arena")
                    .lerp(time);
                vec[idx] = lerped_step;
                visited_indices.insert(idx);
            };

            match collision {
                ArenaCollision::Bound(idx, time) => add_collision(idx, time),
                ArenaCollision::Steps((idx1, idx2), time) => {
                    add_collision(idx1, time);
                    add_collision(idx2, time);
                }
            }
        }

        // fill in the rest of steps that did not collide
        for (idx, vec_step) in vec.iter_mut().enumerate() {
            if visited_indices.contains(&idx as &usize) {
                continue;
            }

            if let Some(step) = self.steps.get(idx) {
                *vec_step = step.clone();
            }
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::Vec2,
        physics::{Collidable, DISTANCE_TOLERANCE},
    };

    use super::*;

    #[test]
    fn test_add() {
        let collidable = Collidable::new(Vec2::new(0.5, 0.5), 0.25);
        let step = Step::new(collidable, Vec2::new(0.25, 0.0));

        let mut arena = Arena::new();
        arena.add(step);

        let vec = arena.into_vec();
        let expected_position = Vec2::new(0.75, 0.5);
        let actual_position = vec
            .get(0)
            .expect("Arena must contain step")
            .resolve()
            .position;

        let diff = (&actual_position - &expected_position).squared_norm();
        assert!(diff < DISTANCE_TOLERANCE);
    }

    #[test]
    fn test_step_collision() {
        let radius = 0.1;
        let delta = Vec2::new(
            1.0 - 2.0 * radius - 2.0 * DISTANCE_TOLERANCE,
            1.0 - 2.0 * radius - 2.0 * DISTANCE_TOLERANCE,
        );

        let lower_bound = radius + DISTANCE_TOLERANCE;
        let upper_bound = 1.0 - radius - DISTANCE_TOLERANCE;

        let collidable1 = Collidable::new(Vec2::new(lower_bound, lower_bound), radius);
        let step1 = Step::new(collidable1, delta.clone());

        let collidable2 = Collidable::new(Vec2::new(upper_bound, upper_bound), radius);
        let step2 = Step::new(collidable2, -delta.clone());

        let mut arena = Arena::new();
        arena.add(step1);
        arena.add(step2);
        let vec = arena.into_vec();

        let resolved_collidable1 = vec
            .get(0)
            .expect("Arena did not produce enough steps")
            .resolve();

        let resolved_collidable2 = vec
            .get(1)
            .expect("Arena did not produce enough steps")
            .resolve();

        let distance =
            (&resolved_collidable1.position - &resolved_collidable2.position).squared_norm();

        assert!(!resolved_collidable1.is_colliding(&resolved_collidable2));
        assert!(distance < 2.0 * (radius + DISTANCE_TOLERANCE));
    }

    #[test]
    fn test_step_collision_tweener() {
        let radius = 0.1;
        let delta = Vec2::new(
            1.0 - 2.0 * radius - 2.0 * DISTANCE_TOLERANCE,
            1.0 - 2.0 * radius - 2.0 * DISTANCE_TOLERANCE,
        );

        let lower_bound = radius + DISTANCE_TOLERANCE;
        let upper_bound = 1.0 - radius - DISTANCE_TOLERANCE;

        let collidable1 = Collidable::new(Vec2::new(lower_bound, lower_bound), radius);
        let step1 = Step::new(collidable1, delta.clone());

        let collidable2 = Collidable::new(Vec2::new(upper_bound, upper_bound), radius);
        let step2 = Step::new(collidable2, -delta.clone());

        let stationary_collidable = Collidable::new(Vec2::new(0.5, 0.5), radius);
        let stationary_step = Step::new(stationary_collidable, Vec2::new(0.0, 0.0));

        let mut arena = Arena::new();
        arena.add(step1);
        arena.add(step2);
        arena.add(stationary_step);
        let vec = arena.into_vec();

        let resolved_collidable1 = vec
            .get(0)
            .expect("Arena did not produce enough steps")
            .resolve();

        let resolved_collidable2 = vec
            .get(1)
            .expect("Arena did not produce enough steps")
            .resolve();

        let stationary_step = vec.get(2).expect("Arena did not produce enough steps");

        assert_eq!(stationary_step.delta.squared_norm(), 0.0);
        let stationary_collidable = stationary_step.resolve();

        assert!(!stationary_collidable.is_colliding(&resolved_collidable1));
        assert!(!stationary_collidable.is_colliding(&resolved_collidable2));

        let distance =
            (&resolved_collidable1.position - &resolved_collidable2.position).squared_norm();
        assert!(distance < 4.0 * (radius + DISTANCE_TOLERANCE));
    }
}

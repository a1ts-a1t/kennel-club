use std::{cmp::Ordering, collections::BinaryHeap};

use crate::physics::{Collidable, Step};

pub struct Arena {
    steps: Vec<Step>,
    heap: BinaryHeap<Collision>,
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

        // schedule the bound collision
        if let Some(time) = new_step.unit_bound_collision_time() {
            self.heap.push(Collision::Bound(new_idx, time));
        }

        // schedule collisions against every step already added
        for (idx, step) in self.steps.iter().enumerate() {
            if let Some(time) = Step::steps_collision_time(step, &new_step) {
                self.heap.push(Collision::Step((idx, new_idx), time));
            }
        }

        self.steps.push(new_step);
    }

    pub fn into_vec(self) -> Vec<Step> {
        let steps = self.steps;
        let mut heap = self.heap;

        // the time each step stopped at (none if still in motion)
        let mut stopped: Vec<Option<f64>> = vec![None; steps.len()];

        // iterator of steps that are still in motion
        let moving_steps = |stopped: &Vec<Option<f64>>| -> Vec<(usize, Step)> {
            stopped
                .iter()
                .enumerate()
                .filter(|(_, stop_time)| stop_time.is_none())
                .map(|(idx, _)| (idx, steps[idx].clone()))
                .collect()
        };

        while let Some(collision) = heap.pop() {
            let time = collision.time();
            if time >= 1.0 {
                break;
            }

            match collision {
                Collision::Bound(idx, _) | Collision::Collidable(idx, _) => {
                    stopped[idx] = match stopped[idx] {
                        Some(_) => continue, // stale collision since step has already been stopped
                        None => Some(time),
                    };

                    let stopped_step = &steps[idx].lerp(time).resolve();

                    let stopped_step_collisions =
                        moving_steps(&stopped)
                            .into_iter()
                            .filter_map(|(idx, step)| {
                                stopped_step_collision_time(&step, stopped_step, time)
                                    .map(|t| (idx, t))
                            });

                    for (idx, t) in stopped_step_collisions {
                        heap.push(Collision::Collidable(idx, t));
                    }
                }
                Collision::Step((idx1, idx2), _) => {
                    match (stopped[idx1], stopped[idx2]) {
                        (None, None) => {
                            stopped[idx1] = Some(time);
                            stopped[idx2] = Some(time);
                        }
                        _ => continue, // one of the stps has already stopped at some point
                    }

                    let stopped_step1 = &steps[idx1].lerp(time).resolve();
                    let stopped_step2 = &steps[idx2].lerp(time).resolve();

                    let stopped_step_collisions =
                        moving_steps(&stopped).into_iter().flat_map(|(idx, step)| {
                            let t1 = stopped_step_collision_time(&step, stopped_step1, time);
                            let t2 = stopped_step_collision_time(&step, stopped_step2, time);

                            match (t1, t2) {
                                (None, None) => vec![],
                                (None, Some(t)) | (Some(t), None) => vec![(idx, t)],
                                (Some(t1), Some(t2)) => vec![(idx, f64::min(t1, t2))],
                            }
                        });

                    for (idx, t) in stopped_step_collisions {
                        heap.push(Collision::Collidable(idx, t));
                    }
                }
            }
        }

        steps
            .iter()
            .enumerate()
            .map(|(idx, step)| match stopped[idx] {
                Some(time) => step.lerp(time),
                None => step.clone(),
            })
            .collect()
    }
}

#[derive(PartialEq)]
enum Collision {
    /// it reaches the arena wall
    Bound(usize, f64),
    /// it and another still-moving step reach each other
    Step((usize, usize), f64),
    /// it reaches the resting position of an already-stopped step
    Collidable(usize, f64),
}

impl Collision {
    fn time(&self) -> f64 {
        match self {
            Collision::Bound(_, t) | Collision::Collidable(_, t) | Collision::Step(_, t) => *t,
        }
    }
}

impl Eq for Collision {}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Collision {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversed so the earliest stop pops out of the max-heap first
        self.time().total_cmp(&other.time()).reverse()
    }
}

fn stopped_step_collision_time(
    step: &Step,
    stopped_step: &Collidable,
    stop_time: f64,
) -> Option<f64> {
    // the rest of the step, starting from the point of the collision in question
    let remainder = Step::new(
        step.lerp(stop_time).resolve(),
        (1.0 - stop_time) * &step.delta,
    );

    // check if that remainder collides with the stopped step
    remainder
        .collidable_collision_time(stopped_step)
        .map(|time| stop_time + time * (1.0 - stop_time)) // normalize time scale
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

    #[test]
    fn test_touching_steps_do_not_pass_through() {
        // a resolved collision leaves the pair right at the contact shell.
        // when they step toward each other again they must stop where they
        // are, not sweep through each other for the whole step
        let radius = 0.05;
        let step1 = Step::new(
            Collidable::new(Vec2::new(0.45, 0.5), radius),
            Vec2::new(0.1, 0.0),
        );
        let step2 = Step::new(
            Collidable::new(Vec2::new(0.55, 0.5), radius),
            Vec2::new(-0.1, 0.0),
        );

        assert_eq!(Step::steps_collision_time(&step1, &step2), Some(0.0));

        let mut arena = Arena::new();
        arena.add(step1);
        arena.add(step2);
        let vec = arena.into_vec();

        let resolved1 = vec[0].resolve();
        let resolved2 = vec[1].resolve();
        assert!(!resolved1.is_colliding(&resolved2));
        assert!(
            (resolved1.position.x - 0.45).abs() < DISTANCE_TOLERANCE,
            "touching creature moved: {:?}",
            resolved1.position
        );
        assert!(
            (resolved2.position.x - 0.55).abs() < DISTANCE_TOLERANCE,
            "touching creature moved: {:?}",
            resolved2.position
        );
    }

    #[test]
    fn test_frozen_step_blocks_later_mover() {
        // A and B collide head-on early in the step and both freeze.
        // C's original trajectory misses BOTH original trajectories
        // (so no collision event is ever registered for it), but it
        // ends the step inside B's resting spot.
        let radius = 0.1;
        let step_a = Step::new(
            Collidable::new(Vec2::new(0.5, 0.28), radius),
            Vec2::new(0.0, 0.1),
        );
        let step_b = Step::new(
            Collidable::new(Vec2::new(0.5, 0.52), radius),
            Vec2::new(0.0, -0.2),
        );
        let step_c = Step::new(
            Collidable::new(Vec2::new(0.45, 0.75), radius),
            Vec2::new(0.05, -0.11),
        );

        let mut arena = Arena::new();
        arena.add(step_a);
        arena.add(step_b);
        arena.add(step_c);

        let vec = arena.into_vec();
        let resolved: Vec<_> = vec.iter().map(|step| step.resolve()).collect();

        for (i, j) in [(0, 1), (0, 2), (1, 2)] {
            assert!(
                !resolved[i].is_colliding(&resolved[j]),
                "steps {i} and {j} overlap: {:?} vs {:?}",
                resolved[i].position,
                resolved[j].position
            );
        }
    }

    #[test]
    fn test_mover_stops_at_late_parking_spot() {
        // B parks on a spot that A has just drifted into. A and B start
        // touching but drift apart so slowly their pair collision would
        // only "end" at t = 1.0, so no meet event ever exists for them.
        // A must still stop when B parks on top of it
        let radius = 0.05;
        let step_a = Step::new(
            Collidable::new(Vec2::new(0.5, 0.6), radius),
            Vec2::new(0.000001, -0.1),
        );
        let step_b = Step::new(
            Collidable::new(Vec2::new(0.5, 0.5), radius),
            Vec2::new(0.0, -0.1),
        );
        let step_c = Step::new(Collidable::new(Vec2::new(0.5, 0.36), radius), Vec2::zero());

        let mut arena = Arena::new();
        arena.add(step_a);
        arena.add(step_b);
        arena.add(step_c);

        let vec = arena.into_vec();
        let resolved: Vec<_> = vec.iter().map(|step| step.resolve()).collect();

        for (i, j) in [(0, 1), (0, 2), (1, 2)] {
            assert!(
                !resolved[i].is_colliding(&resolved[j]),
                "steps {i} and {j} overlap: {:?} vs {:?}",
                resolved[i].position,
                resolved[j].position
            );
        }
    }
}

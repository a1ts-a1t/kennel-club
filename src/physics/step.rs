use crate::{math::Vec2, physics::collidable::Collidable};

#[derive(Clone, Debug)]
pub struct Step {
    pub collidable: Collidable,
    pub delta: Vec2,
}

pub const DISTANCE_TOLERANCE: f64 = 1e-12;

impl Step {
    pub fn new(collidable: Collidable, delta: Vec2) -> Self {
        Self { collidable, delta }
    }

    /**
     * Resolves the step and adds the delta to the collidable's position.
     */
    pub fn resolve(&self) -> Collidable {
        Collidable {
            position: self.collidable.position + self.delta,
            radius: self.collidable.radius,
        }
    }

    pub fn lerp(&self, t: f64) -> Self {
        Self::new(self.collidable, t * &self.delta)
    }

    pub fn unit_bound_collision_time(&self) -> Option<f64> {
        let current_position = &self.collidable.position;

        let final_position = current_position + &self.delta;
        let lower_bound = self.collidable.radius + DISTANCE_TOLERANCE;
        let upper_bound = 1.0 - self.collidable.radius - DISTANCE_TOLERANCE;

        let t_x = if self.delta.x == 0.0 {
            1.0
        } else if final_position.x < lower_bound {
            (lower_bound - current_position.x) / self.delta.x
        } else if final_position.x > upper_bound {
            (upper_bound - current_position.x) / self.delta.x
        } else {
            1.0
        };

        let t_y = if self.delta.y == 0.0 {
            1.0
        } else if final_position.y < lower_bound {
            (lower_bound - current_position.y) / self.delta.y
        } else if final_position.y > upper_bound {
            (upper_bound - current_position.y) / self.delta.y
        } else {
            1.0
        };

        if t_x == 1.0 && t_y == 1.0 {
            return None;
        }

        Some(f64::min(t_x, t_y))
    }

    /**
     * First time in [0, 1.0) that the two steps' trajectories touch.
     * None if they stay clear of each other for the whole step.
     */
    pub fn steps_collision_time(step1: &Self, step2: &Self) -> Option<f64> {
        let (t_enter, t_exit) = collision_roots(step1, step2)?;

        // roots are out of range
        // during entire time step, there is no collision
        if t_exit < 0.0 || t_enter >= 1.0 {
            return None;
        }

        // negative time at entry means that the steps started inside each other
        // if they're getting farther, let them get farther
        if t_enter < 0.0 {
            // the roots sum to -b/a, so a positive sum means closing in
            let approaching = t_enter + t_exit > 0.0;
            return Some(if approaching { 0.0 } else { t_exit });
        }

        Some(t_enter)
    }

    /**
     * The time at which this step collides with a stationary collidable.
     */
    pub fn collidable_collision_time(&self, collidable: &Collidable) -> Option<f64> {
        let other_step = Step::new(*collidable, Vec2::zero());
        Step::steps_collision_time(self, &other_step)
    }
}

/**
 * Entry and exit times of the two steps' contact shells along their full
 * trajectories, or None if the trajectories never touch
 */
fn collision_roots(step1: &Step, step2: &Step) -> Option<(f64, f64)> {
    let delta_diff = &step1.delta - &step2.delta;
    let position_diff = &step1.collidable.position - &step2.collidable.position;
    let radius_sum = step1.collidable.radius + step2.collidable.radius;

    let a = delta_diff.squared_norm();

    // the delta vectors are the same
    // if they weren't colliding before, they won't now
    if a == 0.0 {
        return None;
    }

    // add in tolerance for extra wiggle room
    let c = position_diff.squared_norm() - radius_sum * radius_sum - DISTANCE_TOLERANCE;
    let b = 2.0 * Vec2::dot(&delta_diff, &position_diff);
    let d = b * b - 4.0 * a * c;

    // no roots so the trajectories never touch
    if d < 0.0 {
        return None;
    }

    let d_sq = d.sqrt();
    let t1 = (-b + d_sq) / (2.0 * a);
    let t2 = (-b - d_sq) / (2.0 * a);

    Some((f64::min(t1, t2), f64::max(t1, t2)))
}


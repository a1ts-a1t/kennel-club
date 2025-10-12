use crate::{math::Vec2, physics::collidable::Collidable};

#[derive(Clone, Debug)]
pub struct Step {
    pub collidable: Collidable,
    pub delta: Vec2,
}

pub static DISTANCE_TOLERANCE: f64 = 0.000000000001;

fn next_down_until<F: Fn(f64) -> bool>(t0: f64, f: F) -> f64 {
    let mut t = t0;
    while !f(t0) {
        t = t.next_down();
    }
    t
}

fn next_up_until<F: Fn(f64) -> bool>(t0: f64, f: F) -> f64 {
    let mut t = t0;
    while !f(t0) {
        t = t.next_up();
    }
    t
}

impl Default for Step {
    fn default() -> Self {
        Step::new(Collidable::default(), Vec2::zero())
    }
}

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
        Self::new(self.collidable.clone(), t * &self.delta)
    }

    pub fn unit_bound_collision_time(&self) -> Option<f64> {
        let current_position = &self.collidable.position;

        let final_position = current_position + &self.delta;
        let lower_bound = self.collidable.radius + DISTANCE_TOLERANCE;
        let upper_bound = 1.0 - self.collidable.radius - DISTANCE_TOLERANCE;

        let t_x = if final_position.x < lower_bound {
            (lower_bound - current_position.x) / self.delta.x
        } else if final_position.x > upper_bound {
            (upper_bound - current_position.x) / self.delta.x
        } else {
            1.0
        };

        let t_y = if final_position.y < lower_bound {
            (lower_bound - current_position.y) / self.delta.y
        } else if final_position.y > upper_bound {
            (upper_bound - current_position.y) / self.delta.y
        } else {
            1.0
        };

        if t_x == 1.0 && t_y == 1.0 {
            return None;
        }

        let t = next_down_until(f64::min(t_x, t_y), |t| {
            !self.lerp(t).resolve().is_out_of_unit_bounds() || t <= 0.0
        });

        Some(t)
    }

    pub fn steps_collision_time(step1: &Self, step2: &Self) -> Option<f64> {
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

        // no roots so no collision
        // return ok
        if d < 0.0 {
            return None;
        }

        let f = |t: f64| a * t * t + b * t + c;
        let df = |t: f64| 2.0 * a * t + b;
        let next_time_until = |t0: f64| {
            if df(t0) * f(t0) > 0.0 {
                next_down_until(t0, |t| {
                    !step1
                        .lerp(t)
                        .resolve()
                        .is_colliding(&step2.lerp(t).resolve())
                })
            } else {
                next_up_until(t0, |t| {
                    !step1
                        .lerp(t)
                        .resolve()
                        .is_colliding(&step2.lerp(t).resolve())
                })
            }
        };

        let d_sq = if d <= 0.0 { 0.0 } else { d.sqrt() };
        let t1 = next_time_until((-b + d_sq) / (2.0 * a));
        let t2 = next_time_until((-b - d_sq) / (2.0 * a));

        let (t_min, t_max) = (f64::min(t1, t2), f64::max(t1, t2));

        // roots are out of range
        // during entire time step, there is no collision
        if t_max < 0.0 || t_min >= 1.0 {
            return None;
        }

        // during the entire time step, it is colliding
        // this should really not be the case given the assumptions
        // let them just pass through each other
        // and mark them as no collision
        if t_min < 0.0 && t_max > 1.0 {
            return None;
        }

        Some(if t_min < 0.0 { t_max } else { t_min })
    }
}

impl From<Collidable> for Step {
    fn from(value: Collidable) -> Self {
        Step {
            collidable: value,
            delta: Vec2::zero(),
        }
    }
}

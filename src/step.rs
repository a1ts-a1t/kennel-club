use std::cmp;

use crate::{
    collidable::Collidable,
    math::Vec2
};

#[derive(Clone)]
pub struct Step {
    pub collidable: Collidable,
    delta: Vec2,
}

pub enum ResolutionResult<T> {
    Ok,
    ResolvedTo(T),
    Err,
}

impl Step {
    pub fn new(collidable: Collidable, delta: Vec2) -> Self {
        Self { collidable, delta }
    }

    /**
     * Collapses the step and adds the delta to the collidable's position.
     */
    pub fn collapse(self) -> Collidable {
        Collidable {
            position: self.collidable.position + self.delta,
            radius: self.collidable.radius,
        }
    }

    /**
     * If the collidable is out of bounds, return an error.
     * If the collidable is not ever out of bounds, return ok.
     *
     * Otherwise, return the step whose collidable is the same and delta satisfies the following conditions
     * 1. it magnitude is in [0, |delta|)
     * 2. it is either the zero vector or has the same unit vector as delta
     * 3. position + new delta is within the bounds provided
     * 4. the magnitude of new delta is greater than all other vectors satisfying these conditions
     */
    pub fn resolve_to_bounds(
        &self,
        x_bounds: &(f64, f64),
        y_bounds: &(f64, f64),
    ) -> ResolutionResult<Self> {
        let current_position = &self.collidable.position;
        let radius = &self.collidable.radius;
        let x_radial_bounds = (x_bounds.0 + radius, x_bounds.1 - radius);
        let y_radial_bounds = (y_bounds.0 + radius, y_bounds.1 - radius);

        let in_x_bounds =
            |position: &Vec2| position.x > x_radial_bounds.0 && position.x < x_radial_bounds.1;

        let in_y_bounds =
            |position: &Vec2| position.y > y_radial_bounds.0 && position.y < y_radial_bounds.1;

        if !in_x_bounds(current_position) || !in_y_bounds(current_position) {
            return ResolutionResult::Err;
        }

        let final_position = current_position + &self.delta;

        let t_x = if final_position.x <= x_radial_bounds.0 {
            (x_radial_bounds.0 - current_position.x) / &self.delta.x
        } else if final_position.x >= x_radial_bounds.1 {
            (x_radial_bounds.1 - current_position.x) / &self.delta.x
        } else {
            1.0
        };

        let t_y = if final_position.y <= y_radial_bounds.0 {
            (y_radial_bounds.0 - current_position.y) / &self.delta.y
        } else if final_position.y >= y_radial_bounds.1 {
            (y_radial_bounds.1 - current_position.y) / &self.delta.y
        } else {
            1.0
        };

        if t_x == 1.0 && t_y == 1.0 {
            return ResolutionResult::Ok;
        }

        let t = f64::min(t_x, t_y);
        let delta = (t * &self.delta).next_smaller();
        let step = Step {
            collidable: self.collidable.clone(),
            delta,
        };
        ResolutionResult::ResolvedTo(step)
    }

    /**
     * If the two steps start out colliding, return an error.
     * If the two steps never collide, return ok.
     *
     * Otherwise, return the steps whose collidables are the same and have deltas be t * delta
     * where t satisfies.
     * 1. t in [0, 1)
     * 2. for all t' in [0, t], p1 + t' * delta1 does not collide with p2 + t' * delta2
     * 3. t is greater than all other values satisfying these conditions
     */
    pub fn resolve_steps(step1: &Self, step2: &Self) -> ResolutionResult<(Self, Self)> {
        if step1.collidable.is_colliding(&step2.collidable) {
            return ResolutionResult::Err;
        }

        let delta_diff = &step1.delta - &step2.delta;
        let position_diff = &step1.collidable.position - &step2.collidable.position;
        let radius_sum = &step1.collidable.radius + &step2.collidable.radius;

        let a = delta_diff.squared_norm();
        let b = 2.0 * Vec2::dot(&delta_diff, &position_diff);
        let c = position_diff.squared_norm() - radius_sum * radius_sum;

        let d = b * b - 4.0 * a * c;

        // no roots, so no collision
        if d < 0.0 {
            return ResolutionResult::Ok;
        }

        let d_sq = if d == 0.0 { 0.0 } else { d.sqrt() };
        let t1 = (-b + d_sq) / (2.0 * a);
        let t2 = (-b - d_sq) / (2.0 * a);

        // get minimum value of t that is non-negative
        let t = match (t1.total_cmp(&0.0), t2.total_cmp(&0.0)) {
            // neither is in range so automatic disqualification
            (cmp::Ordering::Less, cmp::Ordering::Less) => f64::MAX,
            (cmp::Ordering::Less, _) => t2, // t1 definitely not in range
            (_, cmp::Ordering::Less) => t1, // t2 definitely not in range
            (_, _) => f64::min(t1, t2),     // both can be in range
        };

        // no roots in range, so no collision
        if t > 1.0 {
            return ResolutionResult::Ok;
        }

        let delta1 = (t * &step1.delta).next_smaller();
        let delta2 = (t * &step2.delta).next_smaller();

        let step1 = Step::new(step1.collidable.clone(), delta1);
        let step2 = Step::new(step2.collidable.clone(), delta2);

        ResolutionResult::ResolvedTo((step1, step2))
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Vec2, is_close};

    use super::*;

    #[test]
    fn test_resolve_to_bounds_err() {
        let step = Step::new(Collidable::new((1.0, 1.0).into(), 2.0), Vec2::zero());
        let (x_bounds, y_bounds) = ((0.0, 1.0), (0.0, 1.0));
        assert!(matches!(
            step.resolve_to_bounds(&x_bounds, &y_bounds),
            ResolutionResult::Err
        ));
    }

    #[test]
    fn test_resolve_to_bounds_ok() {
        let step = Step::new(Collidable::new((2.0, 2.0).into(), 1.0), (0.5, 0.5).into());
        let (x_bounds, y_bounds) = ((0.0, 4.0), (0.0, 4.0));
        match step.resolve_to_bounds(&x_bounds, &y_bounds) {
            ResolutionResult::Ok => (),
            _ => panic!("Expected Ok"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_x() {
        let step = Step::new(Collidable::new((2.0, 2.0).into(), 1.0), (3.0, 0.0).into());
        let (x_bounds, y_bounds) = ((0.0, 4.0), (0.0, 4.0));
        match step.resolve_to_bounds(&x_bounds, &y_bounds) {
            ResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(
                    is_close(&actual_delta.x, &1.0),
                    "Actual delta: {:?}",
                    actual_delta
                );
                assert!(&actual_delta.x < &1.0);
                assert_eq!(actual_delta.y, 0.0);
            }
            _ => panic!("Expected ResolvedTo"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_y() {
        let step = Step::new(Collidable::new((2.0, 2.0).into(), 1.0), (0.0, -3.0).into());
        let (x_bounds, y_bounds) = ((0.0, 4.0), (0.0, 4.0));
        match step.resolve_to_bounds(&x_bounds, &y_bounds) {
            ResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(
                    is_close(&actual_delta.y, &-1.0),
                    "Actual delta: {:?}",
                    actual_delta
                );
                assert!(&actual_delta.y > &-1.0);
                assert_eq!(actual_delta.x, 0.0);
            }
            _ => panic!("Expected ResolvedTo"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_xy() {
        let step = Step::new(Collidable::new((2.0, 2.0).into(), 1.0), (2.0, -3.0).into());
        let (x_bounds, y_bounds) = ((0.0, 4.0), (0.0, 4.0));
        match step.resolve_to_bounds(&x_bounds, &y_bounds) {
            ResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(
                    is_close(&actual_delta.y, &-1.0),
                    "Actual delta: {:?}",
                    actual_delta
                );
                assert!(&actual_delta.y > &-1.0);
                assert!(
                    is_close(&actual_delta.x, &(2.0 / 3.0)),
                    "Actual delta: {:?}",
                    actual_delta
                );
                assert!(&actual_delta.x < &(2.0 / 3.0));
            }
            _ => panic!("Expected ResolvedTo"),
        };
    }

    #[test]
    fn test_resolve_steps_err() {
        let step1 = Step::new(Collidable::new((2.0, 2.0).into(), 1.0), (1.0, 1.0).into());
        let step2 = Step::new(Collidable::new((2.5, 2.5).into(), 1.0), (-1.0, -1.0).into());
        match Step::resolve_steps(&step1, &step2) {
            ResolutionResult::Err => (),
            _ => panic!("Expected Err"),
        }
    }

    #[test]
    fn test_resolve_steps_ok() {
        // these two segments intersect but never at the same time
        let step1 = Step::new(
            Collidable::new((1.0, 1.0).into(), 1.0),
            (100.0, 100.0).into(),
        );
        let step2 = Step::new(
            Collidable::new((100.0, 99.0).into(), 1.0),
            (-100.0, 100.0).into(),
        );
        match Step::resolve_steps(&step1, &step2) {
            ResolutionResult::Ok => (),
            _ => panic!("Expected Ok"),
        }
    }

    #[test]
    fn test_resolve_steps_resolved_to() {
        let step1 = Step::new(Collidable::new((0.0, 0.0).into(), 1.0), (10.0, 10.0).into());
        let step2 = Step::new(
            Collidable::new((10.0, 0.0).into(), 2.0),
            (-10.0, 10.0).into(),
        );
        match Step::resolve_steps(&step1, &step2) {
            ResolutionResult::ResolvedTo((resolved_step1, resolved_step2)) => {
                let collidable1 = resolved_step1.collapse();
                let collidable2 = resolved_step2.collapse();

                let actual_distance = (&collidable1.position - &collidable2.position).norm();

                assert!(is_close(&actual_distance, &3.0));
                assert!(actual_distance > 3.0); // equality would imply collision
            }
            _ => panic!("Expected resolved to"),
        }
    }
}

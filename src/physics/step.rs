use std::cmp;

use crate::{
    math::{Vec2, approx, newtons},
    physics::collidable::Collidable,
};

#[derive(Clone, Debug)]
pub struct Step {
    pub collidable: Collidable,
    delta: Vec2,
}

#[derive(Debug)]
pub enum StepResolutionResult<T> {
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
    pub fn collapse(&self) -> Collidable {
        Collidable {
            position: self.collidable.position.clone() + self.delta.clone(),
            radius: self.collidable.radius,
        }
    }

    /**
     * If the collidable is out of unit bounds, return an error.
     * If the collidable is not ever out of bounds, return ok.
     *
     * Otherwise, return the step whose collidable is the same and delta satisfies the following conditions
     * 1. it magnitude is in [0, |delta|)
     * 2. it is either the zero vector or has the same unit vector as delta
     * 3. position + new delta is within unit bounds
     * 4. the magnitude of new delta is greater than all other vectors satisfying these conditions
     */
    pub fn resolve_to_unit_bounds(&self) -> StepResolutionResult<Self> {
        let current_position = &self.collidable.position;
        let radius = &self.collidable.radius;

        if self.collidable.is_out_of_unit_bounds() {
            return StepResolutionResult::Err;
        }

        let final_position = current_position + &self.delta;
        // let lower_bound = self.collidable.radius;
        let upper_bound = 1.0 - self.collidable.radius;

        let t_x = if approx::lt(&final_position.x, radius) {
            (radius - current_position.x) / &self.delta.x
        } else if approx::gt(&final_position.x, &upper_bound) {
            (upper_bound - current_position.x) / &self.delta.x
        } else {
            1.0
        };

        let t_y = if approx::lt(&final_position.y, radius) {
            (radius - current_position.y) / &self.delta.y
        } else if approx::gt(&final_position.y, &upper_bound) {
            (upper_bound - current_position.y) / &self.delta.y
        } else {
            1.0
        };

        if t_x == 1.0 && t_y == 1.0 {
            return StepResolutionResult::Ok;
        }

        let t = f64::min(t_x, t_y);
        let delta = t * &self.delta;
        let step = Step {
            collidable: self.collidable.clone(),
            delta,
        };
        StepResolutionResult::ResolvedTo(step)
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
    pub fn resolve_steps(step1: &Self, step2: &Self) -> StepResolutionResult<(Self, Self)> {
        if step1.collidable.is_colliding(&step2.collidable) {
            return StepResolutionResult::Err;
        }

        let delta_diff = &step1.delta - &step2.delta;
        let position_diff = &step1.collidable.position - &step2.collidable.position;
        let radius_sum = &step1.collidable.radius + &step2.collidable.radius;

        let a = delta_diff.squared_norm();

        // the delta vectors are the same
        // if they weren't colliding before, they won't now
        if a == 0.0 {
            return StepResolutionResult::Ok;
        }

        let b = 2.0 * Vec2::dot(&delta_diff, &position_diff);
        let c = position_diff.squared_norm() - radius_sum * radius_sum;

        let d = b * b - 4.0 * a * c;

        if d < 0.0 && !step1.collapse().is_colliding(&step2.collapse()) {
            return StepResolutionResult::Ok;
        }

        let t = if d < 0.0 {
            1.0
        } else {
            let d_sq = d.sqrt();
            let t1 = (-b + d_sq) / (2.0 * a);
            let t2 = (-b - d_sq) / (2.0 * a);

            let t_min = match (t1.total_cmp(&0.0), t2.total_cmp(&0.0)) {
                // collision points are before time frame, so no collision occurs
                (cmp::Ordering::Less, cmp::Ordering::Less) => 1.0,
                (cmp::Ordering::Less, _) => t2, // t1 definitely not in range
                (_, cmp::Ordering::Less) => t1, // t2 definitely not in range
                (_, _) => f64::min(t1, t2),     // both can be in range
            };

            t_min.clamp(0.0, 1.0)
        };

        // newton stepping out of weird numerical inaccuracy wonkiness
        let f = |t: f64| approx::round(&(a * t * t + b * t + c), &0.0);
        let df = |t: f64| 2.0 * a * t + b;

        let t_newtons = newtons(t, f, df, (0.0, 1.0));
        let delta1 = t_newtons * &step1.delta;
        let delta2 = t_newtons * &step2.delta;

        let step1 = Step::new(step1.collidable.clone(), delta1);
        let step2 = Step::new(step2.collidable.clone(), delta2);

        if step1.collapse().is_colliding(&step2.collapse()) {
            StepResolutionResult::Err // this really shouldn't happen
        } else {
            StepResolutionResult::ResolvedTo((step1, step2))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec2;

    use super::*;

    #[test]
    fn test_resolve_to_bounds_err() {
        let step = Step::new(Collidable::new((1.0, 1.0).into(), 2.0), Vec2::zero());
        assert!(matches!(
            step.resolve_to_unit_bounds(),
            StepResolutionResult::Err
        ));
    }

    #[test]
    fn test_resolve_to_bounds_ok() {
        let step = Step::new(
            Collidable::new((0.5, 0.5).into(), 0.25),
            (0.25, 0.25).into(),
        );
        match step.resolve_to_unit_bounds() {
            StepResolutionResult::Ok => (),
            _ => panic!("Expected Ok"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_x() {
        let step = Step::new(Collidable::new((0.5, 0.5).into(), 0.25), (1.0, 0.0).into());
        match step.resolve_to_unit_bounds() {
            StepResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(approx::eq(&actual_delta.x, &0.25));
                assert_eq!(actual_delta.y, 0.0);
            }
            _ => panic!("Expected ResolvedTo"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_y() {
        let step = Step::new(Collidable::new((0.5, 0.5).into(), 0.25), (0.0, -1.0).into());
        match step.resolve_to_unit_bounds() {
            StepResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(approx::eq(&actual_delta.y, &-0.25));
                assert_eq!(actual_delta.x, 0.0);
            }
            _ => panic!("Expected ResolvedTo"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_xy() {
        let step = Step::new(Collidable::new((0.5, 0.5).into(), 0.25), (1.0, -2.0).into());
        match step.resolve_to_unit_bounds() {
            StepResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(approx::eq(&actual_delta.y, &-0.25));
                assert!(approx::eq(&actual_delta.x, &0.125));
            }
            _ => panic!("Expected ResolvedTo"),
        };
    }

    #[test]
    fn test_resolve_steps_err() {
        let step1 = Step::new(Collidable::new((2.0, 2.0).into(), 1.0), (1.0, 1.0).into());
        let step2 = Step::new(Collidable::new((2.5, 2.5).into(), 1.0), (-1.0, -1.0).into());
        match Step::resolve_steps(&step1, &step2) {
            StepResolutionResult::Err => (),
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
            StepResolutionResult::Ok => (),
            _ => panic!("Expected Ok"),
        }
    }

    #[test]
    fn test_resolve_steps_resolved_to1() {
        let step1 = Step::new(Collidable::new((0.0, 0.0).into(), 1.0), (10.0, 10.0).into());
        let step2 = Step::new(
            Collidable::new((10.0, 0.0).into(), 2.0),
            (-10.0, 10.0).into(),
        );
        match Step::resolve_steps(&step1, &step2) {
            StepResolutionResult::ResolvedTo((resolved_step1, resolved_step2)) => {
                let collidable1 = resolved_step1.collapse();
                let collidable2 = resolved_step2.collapse();

                let actual_distance = (&collidable1.position - &collidable2.position).norm();
                assert!(approx::eq(&actual_distance, &3.0));
            }
            _ => panic!("Expected resolved to"),
        }
    }

    #[test]
    fn test_resolve_steps_resolved_to2() {
        // from integ tests
        let step1 = Step::new(
            Collidable::new((0.580360474018288, 0.23079649951574432).into(), 0.05),
            (0.0, 0.0).into(),
        );
        let step2 = Step::new(
            Collidable::new((0.5587955392260445, 0.1331494125781081).into(), 0.05),
            (-0.00594648062032381, 0.0632561754298869).into(),
        );
        match Step::resolve_steps(&step1, &step2) {
            StepResolutionResult::ResolvedTo((s1, s2)) => {
                let c1 = s1.collapse();
                let c2 = s2.collapse();
                assert!(!c1.is_colliding(&c2));
            }
            StepResolutionResult::Ok => {
                let c1 = step1.collapse();
                let c2 = step2.collapse();
                assert!(!c1.is_colliding(&c2));
            }
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_resolve_steps_resolved_to3() {
        // from integ tests
        let step1 = Step::new(
            Collidable::new((0.623536166715676, 0.4332076978484418).into(), 0.05),
            (0.006215997585102945, -0.0030123035553920553).into(),
        );
        let step2 = Step::new(
            Collidable::new((0.5478028255026165, 0.36790485695536657).into(), 0.05),
            (-0.001807805394269307, 0.0070150727329173).into(),
        );
        match Step::resolve_steps(&step1, &step2) {
            StepResolutionResult::ResolvedTo((s1, s2)) => {
                let c1 = s1.collapse();
                let c2 = s2.collapse();
                assert!(!c1.is_colliding(&c2));
            }
            StepResolutionResult::Ok => {
                let c1 = step1.collapse();
                let c2 = step2.collapse();
                assert!(!c1.is_colliding(&c2));
            }
            _ => panic!("Unexpected error"),
        }
    }
}

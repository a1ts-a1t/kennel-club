use crate::{collidable::Collidable, vec2::{Delta, Position}};

#[derive(Clone)]
pub struct Step {
    pub collidable: Collidable,
    delta: Delta,
}

pub enum ResolutionResult<T> {
    Ok,
    ResolvedTo(T),
    Err,
}

impl Step {
    pub fn new(collidable: Collidable, delta: Delta) -> Self {
        Self { collidable, delta }
    }

    /**
     * Collapses the step and adds the delta to the collidable's position.
     */
    pub fn collapse(self) -> Collidable {
        Collidable { position: self.collidable.position + self.delta, radius: self.collidable.radius }
    }


    pub fn resolve_to_bounds(&self, x_bounds: &(f64, f64), y_bounds: &(f64, f64)) -> ResolutionResult<Self> {
        let current_position = &self.collidable.position;
        let radius = &self.collidable.radius;
        let x_radial_bounds = (x_bounds.0 + radius, x_bounds.1 - radius);
        let y_radial_bounds = (y_bounds.0 + radius, y_bounds.1 - radius);

        let in_x_bounds = |position: &Position| {
            position.x > x_radial_bounds.0 && position.x < x_radial_bounds.1
        };

        let in_y_bounds = |position: &Position| {
            position.y > y_radial_bounds.0 && position.y < y_radial_bounds.1
        };

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

        let t = f64::max(0.0, f64::min(t_x, t_y).next_down());
        let delta = t * &self.delta;
        let step = Step { collidable: self.collidable.clone(), delta };
        ResolutionResult::ResolvedTo(step)
    }

    pub fn resolve_steps(step1: &Self, step2: &Self) -> ResolutionResult<(Self, Self)> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2::Vec2;

    use super::*;

    static RELATIVE_TOLERANCE: f64 = 0.00001;
    static ABSOLUTE_TOLERANCE: f64 = 0.00000001;

    fn is_close(a: &f64, b: &f64) -> bool {
        (a - b).abs() <= f64::max(RELATIVE_TOLERANCE * f64::max(a.abs(), b.abs()), ABSOLUTE_TOLERANCE)
    }

    #[test]
    fn test_resolve_to_bounds_err() {
        let step = Step::new(Collidable { position: (1.0, 1.0).into(), radius: 2.0 }, Vec2::zero());
        let (x_bounds, y_bounds) = ((0.0, 1.0), (0.0, 1.0));
        assert!(matches!(step.resolve_to_bounds(&x_bounds, &y_bounds), ResolutionResult::Err));
    }

    #[test]
    fn test_resolve_to_bounds_ok() {
        let step = Step::new(Collidable { position: (2.0, 2.0).into(), radius: 1.0 }, (0.5, 0.5).into());
        let (x_bounds, y_bounds) = ((0.0, 4.0), (0.0, 4.0));
        match step.resolve_to_bounds(&x_bounds, &y_bounds) {
            ResolutionResult::Ok => (),
            _ => panic!("Expected Ok"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_x() {
        let step = Step::new(Collidable { position: (2.0, 2.0).into(), radius: 1.0 }, (3.0, 0.0).into());
        let (x_bounds, y_bounds) = ((0.0, 4.0), (0.0, 4.0));
        match step.resolve_to_bounds(&x_bounds, &y_bounds) {
            ResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(is_close(&actual_delta.x, &1.0), "Actual delta: {:?}", actual_delta);
                assert!(&actual_delta.x < &1.0);
                assert_eq!(actual_delta.y, 0.0);
            },
            _ => panic!("Expected ResolvedTo"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_y() {
        let step = Step::new(Collidable { position: (2.0, 2.0).into(), radius: 1.0 }, (0.0, -3.0).into());
        let (x_bounds, y_bounds) = ((0.0, 4.0), (0.0, 4.0));
        match step.resolve_to_bounds(&x_bounds, &y_bounds) {
            ResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(is_close(&actual_delta.y, &-1.0), "Actual delta: {:?}", actual_delta);
                assert!(&actual_delta.y > &-1.0);
                assert_eq!(actual_delta.x, 0.0);
            },
            _ => panic!("Expected ResolvedTo"),
        };
    }

    #[test]
    fn test_resolve_to_bounds_xy() {
        let step = Step::new(Collidable { position: (2.0, 2.0).into(), radius: 1.0 }, (2.0, -3.0).into());
        let (x_bounds, y_bounds) = ((0.0, 4.0), (0.0, 4.0));
        match step.resolve_to_bounds(&x_bounds, &y_bounds) {
            ResolutionResult::ResolvedTo(s) => {
                let actual_delta = s.delta;
                assert!(is_close(&actual_delta.y, &-1.0), "Actual delta: {:?}", actual_delta);
                assert!(&actual_delta.y > &-1.0);
                assert!(is_close(&actual_delta.x, &(2.0/3.0)), "Actual delta: {:?}", actual_delta);
                assert!(&actual_delta.x < &(2.0/3.0));
            },
            _ => panic!("Expected ResolvedTo"),
        };
    }
}


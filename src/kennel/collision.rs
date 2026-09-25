use crate::{
    creature::Creature,
    math::Vec2,
    physics::{Body, Collider, step},
};

pub struct Arena {
    bodies: Vec<Body>,
    smallest: (Option<f64>, Option<f64>), // track the smallest creatures
    fastest: (Option<f64>, Option<f64>),  // track the fastest creatures
    center_of_mass: Vec2,
}

impl Arena {
    pub fn new(center_of_mass: Vec2) -> Self {
        Arena {
            bodies: Vec::new(),
            smallest: (None, None),
            fastest: (None, None),
            center_of_mass,
        }
    }

    pub fn add(&mut self, creature: &Creature) {
        let body = creature.as_body(self.center_of_mass);
        let radius = creature.radius;
        let speed = body.velocity().norm();

        if self.smallest.0.is_none() || radius < self.smallest.0.unwrap() {
            self.smallest = (Some(radius), self.smallest.0);
        } else if self.smallest.1.is_none() || radius < self.smallest.1.unwrap() {
            self.smallest = (self.smallest.0, Some(radius));
        }

        if self.fastest.0.is_none() || speed > self.fastest.0.unwrap() {
            self.fastest = (Some(speed), self.fastest.0);
        } else if self.fastest.1.is_none() || speed > self.fastest.1.unwrap() {
            self.fastest = (self.fastest.0, Some(speed))
        }

        self.bodies.push(body)
    }

    pub fn into_vec(self) -> Result<Vec<Body>, String> {
        let mut bodies = self.bodies;

        let max_closing_speed = match self.fastest {
            (Some(s1), Some(s2)) => Some(s1 + s2),
            _ => None,
        };

        let min_shell_size = match self.smallest {
            (Some(r1), Some(r2)) => Some(r1 + r2),
            _ => None,
        };

        let k = match (max_closing_speed, min_shell_size) {
            (Some(s), Some(r)) => Some((2f64 * s / r).ceil() as usize),
            _ => None,
        };

        let wall = Collider::new_plane(Vec2::new(-1f64, 0f64), 0f64)
            .union(Collider::new_plane(Vec2::new(1f64, 0f64), 1f64))
            .union(Collider::new_plane(Vec2::new(0f64, -1f64), 0f64))
            .union(Collider::new_plane(Vec2::new(0f64, 1f64), 1f64));

        let wall_body = Body::new(wall)
            .with_velocity(Vec2::zero())
            .with_mass(f64::INFINITY);

        bodies.push(wall_body);

        // TODO: no clue what to do with errors still
        let mut stepped_bodies = step(&bodies, k).map_err(|_| "Error resolving collisions")?;
        stepped_bodies.pop(); // remove the wall element

        Ok(stepped_bodies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature::Metadata;
    use crate::physics::signed_distance;

    #[test]
    fn overlapping_creatures_resolve() {
        let creatures: Vec<_> = (0..3)
            .map(|_| {
                let mut creature: crate::creature::Creature = Metadata::mock(0.05).into();
                creature.position = Vec2::new(0.5, 0.5);
                creature
            })
            .collect();

        let mut arena = Arena::new(Vec2::new(0.5, 0.5));
        for creature in &creatures {
            arena.add(creature);
        }

        let bodies = arena
            .into_vec()
            .expect("arena must resolve overlapping creatures");
        assert_eq!(bodies.len(), 3);

        for (i, a) in bodies.iter().enumerate() {
            let (p, r) = match a.collider() {
                Collider::Circle { center, radius } => (center, radius),
                other => panic!("creature {i} came back as {other:?}"),
            };
            assert!(
                p.x >= r && p.x <= 1.0 - r && p.y >= r && p.y <= 1.0 - r,
                "creature {i} escaped at ({}, {})",
                p.x,
                p.y
            );

            for (j, b) in bodies.iter().enumerate().skip(i + 1) {
                let gap = signed_distance(&a.collider(), &b.collider())
                    .expect("gap must exist between circles");
                assert!(gap > 0.0, "creatures {i} and {j} ended in contact");
            }
        }
    }
}

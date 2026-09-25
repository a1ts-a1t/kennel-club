use crate::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum Collider {
    HalfPlane { normal: Vec2, offset: f64 },
    Circle { center: Vec2, radius: f64 },
}

impl Collider {
    pub fn new_plane(normal: Vec2, offset: f64) -> Self {
        Collider::HalfPlane {
            normal: normal.normalized(),
            offset,
        }
    }

    pub fn new_circle(center: Vec2, radius: f64) -> Self {
        Collider::Circle { center, radius }
    }

    /// normal of the surface wrt a point, oriented to direction
    /// of greatest signed distance increase
    /// always a unit vector
    pub fn normal(&self, p: Vec2) -> Vec2 {
        match self {
            Collider::HalfPlane { normal, offset: _ } => -*normal,
            Collider::Circle { center, radius: _ } => {
                if p != *center {
                    (p - *center).normalized()
                } else {
                    Vec2::new(1f64, 0f64)
                }
            }
        }
    }

    pub fn signed_distance(&self, p: Vec2) -> f64 {
        match self {
            Collider::HalfPlane { normal, offset } => offset - Vec2::dot(p, *normal),
            Collider::Circle { center, radius } => (p - *center).norm() - radius,
        }
    }

    pub fn translate(&self, p: Vec2) -> Self {
        match self {
            Collider::HalfPlane { normal, offset } => Collider::HalfPlane {
                normal: *normal,
                offset: offset + Vec2::dot(p, *normal),
            },
            Collider::Circle { center, radius } => Collider::Circle {
                center: *center + p,
                radius: *radius,
            },
        }
    }
}

/// inf(norm(p - q)) where
/// p satisfies a.signed_distance(p) <= 0
/// q satisfies b.signed_distance(q) <= 0
/// in other words, the smallest signed distance between two
/// colliders
pub fn signed_distance(a: Collider, b: Collider) -> Option<f64> {
    match (a, b) {
        (Collider::Circle { center, radius }, other)
        | (other, Collider::Circle { center, radius }) => {
            Some(other.signed_distance(center) - radius)
        }
        (
            Collider::HalfPlane {
                normal: normal_a,
                offset: offset_a,
            },
            Collider::HalfPlane {
                normal: normal_b,
                offset: offset_b,
            },
        ) => {
            // non parallel half planes always intersect
            // if the half planes have the same normal, one is a subset of the other
            // furthermore, there is no smallest signed distance
            if normal_a != -normal_b {
                return None;
            }

            Some(offset_a + offset_b)
        }
    }
}

/// gradient with respect to v of a.translate(v).signed_distance(b)
/// evaluated at v = 0
pub fn normal(a: Collider, b: Collider) -> Option<Vec2> {
    match (a, b) {
        (
            Collider::HalfPlane {
                normal: normal_a,
                offset: _,
            },
            Collider::HalfPlane {
                normal: normal_b,
                offset: _,
            },
        ) => {
            if normal_a != -normal_b {
                return None; // non-antinormal half planes are always intersecting
            }

            Some(normal_a)
        }
        (hp @ Collider::HalfPlane { .. }, Collider::Circle { center, radius: _ }) => {
            Some(-hp.normal(center))
        }
        (Collider::Circle { center, radius: _ }, other) => Some(other.normal(center)),
    }
}

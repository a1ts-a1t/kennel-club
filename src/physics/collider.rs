use crate::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum Collider {
    HalfPlane { normal: Vec2, offset: f64 },
    Circle { center: Vec2, radius: f64 },
}

impl Collider {
    /// normal of the surface wrt a point, oriented to direction
    /// of greatest signed distance increase
    /// always a unit vector
    pub fn normal(&self, p: Vec2) -> Vec2 {
        let direction = match self {
            Collider::HalfPlane { normal, offset: _ } => *normal,
            Collider::Circle { center, radius: _ } => p - *center,
        };

        direction.normalized()
    }

    pub fn signed_distance(&self, p: Vec2) -> f64 {
        match self {
            Collider::HalfPlane { normal, offset } => offset - Vec2::dot(p, normal.normalized()),
            Collider::Circle { center, radius } => (p - *center).norm() - radius,
        }
    }

    pub fn translate(&self, p: Vec2) -> Self {
        match self {
            Collider::HalfPlane { normal, offset } => {
                let n = normal.normalized();
                Collider::HalfPlane {
                    normal: n,
                    offset: offset + Vec2::dot(p, n),
                }
            }
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
pub fn signed_distance(a: Collider, b: Collider) -> f64 {
    match (a, b) {
        (Collider::Circle { center, radius }, other @ _)
        | (other @ _, Collider::Circle { center, radius }) => {
            other.signed_distance(center) - radius
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
            if normal_a != -normal_b {
                return 0f64;
            }

            // if the offsets are equal, then they meet at the boundary
            if offset_a == offset_b {
                return 0f64;
            }

            offset_a + offset_b
        }
    }
}

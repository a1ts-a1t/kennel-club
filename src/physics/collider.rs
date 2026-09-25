use crate::math::Vec2;

#[derive(Clone, Debug)]
pub enum Collider {
    HalfPlane { normal: Vec2, offset: f64 },
    Circle { center: Vec2, radius: f64 },
    Union(Box<Collider>, Box<Collider>),
}

impl Collider {
    pub fn new_plane(normal: Vec2, offset: f64) -> Self {
        Collider::HalfPlane {
            normal: normal.normalized(),
            offset: offset,
        }
    }

    pub fn new_circle(center: Vec2, radius: f64) -> Self {
        Collider::Circle { center, radius }
    }

    pub fn union(self, other: Collider) -> Self {
        Collider::Union(Box::new(self), Box::new(other))
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
            Collider::Union(a, b) => {
                let sigma_a = a.signed_distance(p);
                let sigma_b = b.signed_distance(p);

                if sigma_a < sigma_b {
                    a.normal(p)
                } else {
                    b.normal(p)
                }
            }
        }
    }

    pub fn signed_distance(&self, p: Vec2) -> f64 {
        match self {
            Collider::HalfPlane { normal, offset } => offset - Vec2::dot(p, *normal),
            Collider::Circle { center, radius } => (p - *center).norm() - radius,
            Collider::Union(a, b) => f64::min(a.signed_distance(p), b.signed_distance(p)),
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
            Collider::Union(a, b) => a.translate(p).union(b.translate(p)),
        }
    }
}

pub fn union(a: Collider, b: Collider) -> Collider {
    a.union(b)
}

/// inf(norm(p - q)) where
/// p satisfies a.signed_distance(p) <= 0
/// q satisfies b.signed_distance(q) <= 0
/// in other words, the smallest signed distance between two
/// colliders
pub fn signed_distance(a: &Collider, b: &Collider) -> Option<f64> {
    match (a, b) {
        (Collider::Union(a, b), other @ _) | (other @ _, Collider::Union(a, b)) => Some(f64::min(
            signed_distance(a, other)?,
            signed_distance(b, other)?,
        )),
        (Collider::Circle { center, radius }, other @ _)
        | (other @ _, Collider::Circle { center, radius }) => {
            Some(other.signed_distance(*center) - radius)
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
            if *normal_a != -*normal_b {
                return None;
            }

            Some(offset_a + offset_b)
        }
    }
}

/// gradient with respect to v of a.translate(v).signed_distance(b)
/// evaluated at v = 0
pub fn normal(a: &Collider, b: &Collider) -> Option<Vec2> {
    match (a, b) {
        (Collider::Union(a, b), other @ _) => {
            let sigma_a = signed_distance(a, other)?;
            let sigma_b = signed_distance(b, other)?;

            if sigma_a < sigma_b {
                normal(a, other)
            } else {
                normal(b, other)
            }
        }
        (other @ _, Collider::Union(a, b)) => {
            let sigma_a = signed_distance(a, other)?;
            let sigma_b = signed_distance(b, other)?;

            if sigma_a < sigma_b {
                normal(other, a)
            } else {
                normal(other, b)
            }
        }
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
            if *normal_a != -*normal_b {
                return None; // non-antinormal half planes are always intersecting
            }

            Some(*normal_a)
        }
        (hp @ Collider::HalfPlane { .. }, Collider::Circle { center, radius: _ }) => {
            Some(-hp.normal(*center))
        }
        (Collider::Circle { center, radius: _ }, other @ _) => Some(other.normal(*center)),
    }
}

use std::cmp::Ordering;

pub enum RealQuadraticRoots {
    Double(f64, f64),
    Single(f64),
    None,
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> RealQuadraticRoots {
    let discriminant = b * b - 4f64 * a * c;
    match discriminant.partial_cmp(&0f64).unwrap() {
        Ordering::Less => RealQuadraticRoots::None,
        Ordering::Equal => RealQuadraticRoots::Single(-b / (2f64 * a)),
        Ordering::Greater => {
            let sqrt_discriminant = discriminant.sqrt();
            let root1 = (-b + sqrt_discriminant) / (2f64 * a);
            let root2 = (-b - sqrt_discriminant) / (2f64 * a);
            RealQuadraticRoots::Double(root1, root2)
        }
    }
}


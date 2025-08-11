pub use vec2::Vec2;

mod vec2;


static RELATIVE_TOLERANCE: f64 = 0.00001;
static ABSOLUTE_TOLERANCE: f64 = 0.00000001;

pub fn is_close(a: &f64, b: &f64) -> bool {
    (a - b).abs()
        <= f64::max(
            RELATIVE_TOLERANCE * f64::max(a.abs(), b.abs()),
            ABSOLUTE_TOLERANCE,
        )
}


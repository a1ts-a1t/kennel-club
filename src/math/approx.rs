use std::cmp;

static RELATIVE_TOLERANCE: f64 = 0.00001;
static ABSOLUTE_TOLERANCE: f64 = 0.00000001;

pub fn eq(a: &f64, b: &f64) -> bool {
    (a - b).abs()
        <= f64::max(
            RELATIVE_TOLERANCE * f64::max(a.abs(), b.abs()),
            ABSOLUTE_TOLERANCE,
        )
}

/**
 * If the target is approxiamtely equal to the value,
 * return the target. Otherwise, return the value.
 */
pub fn round(val: &f64, target: &f64) -> f64 {
    if eq(val, target) {
        target.clone()
    } else {
        val.clone()
    }
}

/**
 * Not close and less than
 */
pub fn lt(a: &f64, b: &f64) -> bool {
    !eq(a, b) && a < b
}

/**
 * Not close and greater than
 */
pub fn gt(a: &f64, b: &f64) -> bool {
    !eq(a, b) && a > b
}

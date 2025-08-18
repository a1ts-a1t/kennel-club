static MAX_NEWTON_ITERS: u16 = 1000;

pub fn newtons<F, DF>(x_0: f64, f: F, df: DF) -> Result<f64, ()>
where
    F: Fn(f64) -> f64,
    DF: Fn(f64) -> f64,
{
    let mut x_n = x_0;
    for _ in 0..MAX_NEWTON_ITERS {
        let f_x_n = f(x_n);
        if f_x_n == 0.0 {
            return Ok(x_n);
        }

        let df_x_n = df(x_n);
        if df_x_n == 0.0 {
            return Err(());
        }

        x_n = x_n - f_x_n / df_x_n;
    }

    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtons() {
        let f = |x: f64| x * x - 1.0;
        let df = |x: f64| 2.0 * x;

        let root = newtons(64.0, f, df).unwrap();

        assert_eq!(0.0, f(root));
    }
}

use std::ops::Range;

/// Returns the indefinite integral of a function.
pub fn integral<F>(f: F, dx: f64) -> impl Fn(Range<f64>) -> f64
where
    F: Fn(f64) -> f64,
{
    move |range: Range<f64>| {
        let range_len = range.end - range.start;
        let steps_count = (range_len / dx).floor();
        let dx = range_len / steps_count;
        let steps_count = steps_count as u64;

        let mut sum = 0.0;
        let start_at = range.start + dx / 2.0; // Aligns the samples at the center of the range
        for i in 0..steps_count {
            let x = start_at + i as f64 * dx;
            sum += f(x) * dx;
        }

        sum
    }
}

/// Gets the integral of a function from -inf to +inf using the substitution technique.
pub fn reals_integral<F>(f: F, dx: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    // Function that maps [0,1] to ]-inf, +inf[
    fn g(x: f64) -> f64 {
        (x / (1.0 - x)).ln()
    }

    // Derivative of g
    fn g_(x: f64) -> f64 {
        1.0 / (x * (1.0 - x))
    }

    let composite = |x| f(g(x)) * g_(x);
    let integral = integral(composite, dx);

    return integral(0.0..1.0);
}

/// Gets the average value of a function in the set of real numbers.
pub fn avg_value<F>(f: F, dx: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let universe = reals_integral(&f, dx);
    let sxfxdx = |x| x * f(x);

    reals_integral(sxfxdx, dx) / universe
}

pub fn stable_power(base: f64, exp: f64) -> Option<f64> {
    // prevent power producing a number too close to 1 to accurately distinguish from 1.
    let num = base.powf(exp);
    if (num - 1.0).abs() < 1e-6 {
        None
    } else {
        Some(num)
    }
}

pub fn fast_summation(num: f64) -> f64 {
    num / 2. * (num + 1.)
}
pub fn fast_factorial(num: f64) -> f64 {
    // we can precompute since its only 18! values
    const FACTORIALS: [f64; 18] = [
        1.,
        1.,
        2.,
        6.,
        24.,
        120.,
        720.,
        5040.,
        40320.,
        362880.,
        3628800.,
        39916800.,
        479001600.,
        6227020800.,
        87178291200.,
        1307674368000.,
        20922789888000.,
        355687428096000.,
    ];
    FACTORIALS[num as usize]
}

pub fn stable_add(left: f64, right: f64) -> Option<f64> {
    // check if either number has a decimal
    let (left_frac, right_frac) = (left.fract() != 0., right.fract() != 0.);
    match (left_frac, right_frac) {
        // if both numbers have a decimal, or neither number has a decimal, we can just add them
        (true, true) | (false, false) => Some(left + right),
        // if only one number has a decimal, it only counts if the result also has a decimal
        (true, false) | (false, true) => {
            let result = left + right;
            if result.fract() != 0. {
                Some(result)
            } else {
                None
            }
        }
    }
}

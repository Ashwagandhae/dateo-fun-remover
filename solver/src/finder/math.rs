const ROUNDING_ERROR: f64 = 0.0000000001;
const MAX_NUM_SIZE: f64 = 1e15;

const POWER_DELTA: f64 = 0.00001;
const DIVIDE_DELTA: f64 = 0.00001;

// based off https://dateo-math-game.com/inputLogic.js
fn within_rounding_error(x: f64, y: f64, delta: f64) -> bool {
    y - delta <= x && x <= delta + y
}

pub fn add(left: f64, right: f64) -> Option<f64> {
    Some(left + right)
}
pub fn subtract(left: f64, right: f64) -> Option<f64> {
    Some(left - right)
}

pub fn multiply(left: f64, right: f64) -> Option<f64> {
    Some(left * right)
}

pub fn divide(left: f64, right: f64) -> Option<f64> {
    if right == 0. {
        return None;
    }
    let res = left / right;
    if within_rounding_error(res, 0., DIVIDE_DELTA) && left != 0. {
        return None;
    }
    Some(res)
}

pub fn power(left: f64, right: f64) -> Option<f64> {
    if left == 0. && right == 0. {
        return None;
    }
    let res = left.powf(right);
    if res.is_nan() {
        if within_rounding_error(1. / right % 2., 1., POWER_DELTA) {
            return Some(-((-left).powf(right)));
        }
        return None;
    }
    if within_rounding_error(res, 1., POWER_DELTA) && left.abs() != 1. && right != 0. {
        return None;
    }
    if within_rounding_error(res, 0., POWER_DELTA) && left != 0. {
        return None;
    }
    Some(res)
}

pub fn root(left: f64, right: f64) -> Option<f64> {
    if left == 0. {
        return None;
    }
    let exponent = 1. / left;
    if within_rounding_error(exponent, 0., POWER_DELTA) {
        return None;
    }
    power(right, exponent)
}

pub fn square_root(num: f64) -> Option<f64> {
    Some(power(num, 0.5)?)
}

pub fn summation(num: f64) -> Option<f64> {
    if num < 0. || num.fract().abs() > ROUNDING_ERROR {
        return None;
    }
    Some(0.5 * num * (num + 1.))
}

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
pub fn factorial(num: f64) -> Option<f64> {
    if num < 0. || num.fract().abs() > ROUNDING_ERROR {
        return None;
    }

    FACTORIALS.get(num as usize).copied()
}

// functions not from the game

pub fn square_root_rev(num: f64) -> Option<f64> {
    if num < 0. {
        return None;
    }
    Some(power(num, 2.)?)
}

pub fn summation_rev(num: f64) -> Option<f64> {
    if num < 0. || num.fract().abs() > ROUNDING_ERROR {
        return None;
    }
    Some((-1. + (1. + 8. * num).sqrt()) / 2.).filter(|x| x.fract().abs() < ROUNDING_ERROR)
}

pub fn factorial_rev(num: f64) -> Option<f64> {
    if num < 0. || num.fract().abs() > ROUNDING_ERROR {
        return None;
    }
    FACTORIALS.iter().position(|&x| x == num).map(|x| x as f64)
}

// left + right = res
// right = res - left
pub fn add_rev_left(left: f64, res: f64) -> Option<f64> {
    Some(res - left)
}

// left - right = res
// right = left - res
pub fn subtract_rev_left(left: f64, res: f64) -> Option<f64> {
    Some(left - res)
}
// left = right + res
pub fn subtract_rev_right(right: f64, res: f64) -> Option<f64> {
    Some(right + res)
}

// left * right = res
// right = res / left
pub fn multiply_rev_left(left: f64, res: f64) -> Option<f64> {
    Some(res / left)
}

// left / right = res
// right = left / res
pub fn divide_rev_left(left: f64, res: f64) -> Option<f64> {
    Some(left / res)
}
// left = right * res
pub fn divide_rev_right(right: f64, res: f64) -> Option<f64> {
    Some(right * res)
}

// left ^ right = res
// right = log_base(res, left)
pub fn power_rev_left(left: f64, res: f64) -> Option<f64> {
    Some(res.log(left))
}
// left = res ^ (1 / right)
pub fn power_rev_right(right: f64, res: f64) -> Option<f64> {
    Some(res.powf(1. / right))
}

// left ^ (1 / right) = res
// right = log_base(left, res)
pub fn root_rev_left(left: f64, res: f64) -> Option<f64> {
    Some(left.log(res))
}
// left = res ^ right
pub fn root_rev_right(right: f64, res: f64) -> Option<f64> {
    Some(res.powf(right))
}

pub fn within_error(test: f64, goal: f64) -> bool {
    (test - goal).abs() < ROUNDING_ERROR
}

pub fn within_limit(num: &f64) -> bool {
    num.abs() < MAX_NUM_SIZE
}

use crate::finder::math::{
    factorial, factorial_reverse, square_root, square_root_reverse, summation, summation_reverse,
    within_limit,
};
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum Func {
    SquareRoot,
    Factorial,
    Summation,
}

impl Func {
    pub fn apply_no_limit(&self, num: f64) -> Option<f64> {
        match self {
            Func::SquareRoot => square_root(num),
            Func::Factorial => factorial(num),
            Func::Summation => summation(num),
        }
        .filter(|res| !res.is_nan())
        // prevent functions from doing nothing
        .filter(|res| *res != num)
    }
    pub fn apply(&self, num: f64) -> Option<f64> {
        self.apply_no_limit(num).filter(within_limit)
    }
    pub fn apply_if_limit(&self, num: f64, limit: bool) -> Option<f64> {
        if limit {
            self.apply(num)
        } else {
            self.apply_no_limit(num)
        }
    }
    pub fn apply_reverse(&self, num: f64) -> Option<f64> {
        match self {
            Func::SquareRoot => square_root_reverse(num),
            Func::Factorial => factorial_reverse(num),
            Func::Summation => summation_reverse(num),
        }
        .filter(|res| !res.is_nan())
        .filter(within_limit)
        // prevent functions from doing nothing
        .filter(|res| *res != num)
    }
    pub fn apply_reverse_if(&self, num: f64, reverse: bool) -> Option<f64> {
        if reverse {
            self.apply_reverse(num)
        } else {
            self.apply(num)
        }
    }
    pub fn is_behind(&self) -> bool {
        match self {
            Func::SquareRoot => false,
            Func::Factorial => true,
            Func::Summation => false,
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Func::SquareRoot => write!(f, "²√"),
            Func::Factorial => write!(f, "!"),
            Func::Summation => write!(f, "Σ"),
        }
    }
}

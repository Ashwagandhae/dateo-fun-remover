use crate::finder::math::{
    factorial, factorial_rev, square_root, square_root_rev, summation, summation_rev, within_limit,
};
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter, PartialEq)]
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
    pub fn apply_rev(&self, num: f64) -> Option<f64> {
        match self {
            Func::SquareRoot => square_root_rev(num),
            Func::Factorial => factorial_rev(num),
            Func::Summation => summation_rev(num),
        }
        .filter(|res| !res.is_nan())
        .filter(within_limit)
        // prevent functions from doing nothing
        .filter(|res| *res != num)
    }
    pub fn apply_rev_if(&self, num: f64, rev: bool) -> Option<f64> {
        if rev {
            self.apply_rev(num)
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

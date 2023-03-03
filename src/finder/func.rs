use crate::finder::math::{
    factorial, factorial_reversed, is_valid_num, square_root, square_root_reversed, summation,
    summation_reversed,
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
    pub fn apply(&self, num: f64) -> Option<f64> {
        match self {
            Func::SquareRoot => square_root(num),
            Func::Factorial => factorial(num),
            Func::Summation => summation(num),
        }
        .filter(is_valid_num)
        // prevent functions from doing nothing
        .filter(|res| *res != num)
    }
    pub fn apply_reversed(&self, num: f64) -> Option<f64> {
        match self {
            Func::SquareRoot => square_root_reversed(num),
            Func::Factorial => factorial_reversed(num),
            Func::Summation => summation_reversed(num),
        }
        .filter(is_valid_num)
        // prevent functions from doing nothing
        .filter(|res| *res != num)
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

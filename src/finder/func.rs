use crate::finder::math::{factorial, is_valid_num, square_root, summation};
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

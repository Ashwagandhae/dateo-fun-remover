use crate::finder::math::{fast_factorial, fast_summation, stable_power};
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum Function {
    SquareRoot,
    Factorial,
    Summation,
}

impl Function {
    pub fn apply(&self, n: f64) -> Option<f64> {
        let res = match self {
            Function::SquareRoot => {
                if n < 0. {
                    None
                } else {
                    stable_power(n, 0.5)
                }
            }
            Function::Factorial => {
                // no negative factorials, no non-integer factorials, no factorials larger than 18! (because 18! is larger than 10^15)
                if n < 0. || n.fract() != 0. || n >= 18.0 {
                    None
                } else {
                    Some(fast_factorial(n))
                }
            }
            Function::Summation => {
                if n < 0. || n.fract() != 0. {
                    None
                } else {
                    Some(fast_summation(n))
                }
            }
        };
        res.filter(|res| 
            // res cant be larger than 10^15
            res.abs() <= 1e15 && 
            // function has to actually do something
            *res != n 
            // res cant be nan or inf
            && !res.is_nan() && !res.is_infinite())
    }
    pub fn is_behind(&self) -> bool {
        match self {
            Function::SquareRoot => false,
            Function::Factorial => true,
            Function::Summation => false,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::SquareRoot => write!(f, "²√"),
            Function::Factorial => write!(f, "!"),
            Function::Summation => write!(f, "Σ"),
        }
    }
}

use crate::finder::math::*;

use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root,
}

impl Operation {
    pub fn apply_no_limit(&self, left: f64, right: f64) -> Option<f64> {
        match self {
            Operation::Add => add(left, right),
            Operation::Subtract => subtract(left, right),
            Operation::Multiply => multiply(left, right),
            Operation::Divide => divide(left, right),
            Operation::Power => power(left, right),
            Operation::Root => root(left, right),
        }
        .filter(|res| !res.is_nan())
    }
    pub fn apply(&self, left: f64, right: f64) -> Option<f64> {
        self.apply_no_limit(left, right).filter(within_limit)
    }
    pub fn apply_if_limit(&self, left: f64, right: f64, limit: bool) -> Option<f64> {
        if limit {
            self.apply(left, right)
        } else {
            self.apply_no_limit(left, right)
        }
    }
    pub fn apply_reverse_left(&self, left: f64, res: f64) -> Option<f64> {
        match self {
            Operation::Add => add_reverse_left(left, res),
            Operation::Subtract => subtract_reverse_left(left, res),
            Operation::Multiply => multiply_reverse_left(left, res),
            Operation::Divide => divide_reverse_left(left, res),
            Operation::Power => power_reverse_left(left, res),
            Operation::Root => root_reverse_left(left, res),
        }
        .filter(|res| !res.is_nan())
        .filter(within_limit)
    }

    pub fn apply_reverse_right(&self, right: f64, res: f64) -> Option<f64> {
        match self {
            Operation::Add => add_reverse_right(right, res),
            Operation::Subtract => subtract_reverse_right(right, res),
            Operation::Multiply => multiply_reverse_right(right, res),
            Operation::Divide => divide_reverse_right(right, res),
            Operation::Power => power_reverse_right(right, res),
            Operation::Root => root_reverse_right(right, res),
        }
        .filter(|res| !res.is_nan())
        .filter(within_limit)
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            Operation::Add | Operation::Multiply => true,
            _ => false,
        }
    }
}
impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "/"),
            Operation::Power => write!(f, "^"),
            Operation::Root => write!(f, "√"),
        }
    }
}

use crate::finder::math::{add, divide, multiply, power, root, subtract, within_limit};

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

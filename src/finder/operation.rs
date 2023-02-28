use crate::finder::math::{add, divide, is_valid_num, multiply, power, root, subtract};

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
    pub fn apply(&self, left: f64, right: f64) -> Option<f64> {
        match self {
            Operation::Add => add(left, right),
            Operation::Subtract => subtract(left, right),
            Operation::Multiply => multiply(left, right),
            Operation::Divide => divide(left, right),
            Operation::Power => power(left, right),
            Operation::Root => root(left, right),
        }
        .filter(is_valid_num)
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

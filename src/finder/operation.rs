use crate::finder::math::{stable_add, stable_power};

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
            Operation::Add => stable_add(left, right),
            Operation::Subtract => stable_add(left, -right),
            Operation::Multiply => Some(left * right),
            Operation::Divide => Some(left / right),
            Operation::Power => stable_power(left, right),
            Operation::Root => stable_power(right, 1.0 / left),
        }
        .filter(|num| num.abs() <= 1e15 && !num.is_nan() && !num.is_infinite())
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

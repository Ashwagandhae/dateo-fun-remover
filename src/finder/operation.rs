use crate::finder::math::*;

use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum Operation {
    Add,
    Multiply,
    Subtract,
    SubtractSwitch,
    Divide,
    DivideSwitch,
    Power,
    PowerSwitch,
    Root,
    RootSwitch,
}

impl Operation {
    #[inline(never)]
    pub fn apply_all(
        num: f64,
        other_num: f64,
        rev: bool,
    ) -> impl Iterator<Item = (Operation, f64)> {
        Operation::iter().flat_map(move |op| {
            if rev {
                op.apply_rev(num, other_num)
            } else {
                op.apply(num, other_num)
            }
            .map(|num| (op.clone(), num))
        })
    }

    pub fn apply_no_limit(&self, left: f64, right: f64) -> Option<f64> {
        match self {
            Operation::Add => add(left, right),
            Operation::Multiply => multiply(left, right),
            Operation::Subtract => subtract(left, right),
            Operation::SubtractSwitch => subtract(right, left),
            Operation::Divide => divide(left, right),
            Operation::DivideSwitch => divide(right, left),
            Operation::Power => power(left, right),
            Operation::PowerSwitch => power(right, left),
            Operation::Root => root(left, right),
            Operation::RootSwitch => root(right, left),
        }
        .filter(|res| !res.is_nan())
    }
    pub fn apply(&self, left: f64, right: f64) -> Option<f64> {
        self.apply_no_limit(left, right).filter(within_limit)
    }
    pub fn score(&self) -> u32 {
        match self {
            Operation::Power | Operation::Root | Operation::PowerSwitch | Operation::RootSwitch => {
                1
            }
            _ => 0,
        }
    }
    pub fn apply_if_limit(&self, left: f64, right: f64, limit: bool) -> Option<f64> {
        if limit {
            self.apply(left, right)
        } else {
            self.apply_no_limit(left, right)
        }
    }
    pub fn apply_rev(&self, num: f64, res: f64) -> Option<f64> {
        match self {
            Operation::Add => add_rev_left(num, res),
            Operation::Multiply => multiply_rev_left(num, res),
            Operation::Subtract => subtract_rev_left(num, res),
            Operation::SubtractSwitch => subtract_rev_right(num, res),
            Operation::Divide => divide_rev_left(num, res),
            Operation::DivideSwitch => divide_rev_right(num, res),
            Operation::Power => power_rev_left(num, res),
            Operation::PowerSwitch => power_rev_right(num, res),
            Operation::Root => root_rev_left(num, res),
            Operation::RootSwitch => root_rev_right(num, res),
        }
        .filter(|res| !res.is_nan())
        .filter(within_limit)
    }

    // pub fn is_commutative(&self) -> bool {
    //     match self {
    //         Operation::Add | Operation::Multiply => true,
    //         _ => false,
    //     }
    // }
    pub fn is_switched(&self) -> bool {
        match self {
            Operation::SubtractSwitch
            | Operation::DivideSwitch
            | Operation::PowerSwitch
            | Operation::RootSwitch => true,
            _ => false,
        }
    }
}
impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Multiply => write!(f, "*"),
            Operation::Subtract => write!(f, "-"),
            Operation::SubtractSwitch => write!(f, "-"),
            Operation::Divide => write!(f, "/"),
            Operation::DivideSwitch => write!(f, "/"),
            Operation::Power => write!(f, "^"),
            Operation::PowerSwitch => write!(f, "^"),
            Operation::Root => write!(f, "√"),
            Operation::RootSwitch => write!(f, "√"),
        }
    }
}

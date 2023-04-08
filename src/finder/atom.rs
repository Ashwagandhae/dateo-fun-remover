use itertools::Itertools;

use super::func_list::FuncList;
use super::math::within_error;
use std::fmt::{Display, Formatter};

use super::operation::Operation;

#[derive(Debug, Clone)]
pub enum Val {
    Num(f64),
    Express {
        left: Box<Atom>,
        right: Box<Atom>,
        op: Operation,
    },
}
impl From<Val> for Atom {
    fn from(val: Val) -> Self {
        Atom::new(val)
    }
}

impl From<f64> for Val {
    fn from(n: f64) -> Self {
        Val::Num(n)
    }
}
impl From<&f64> for Val {
    fn from(n: &f64) -> Self {
        Val::Num(*n)
    }
}
#[derive(Debug, Clone)]
pub struct Atom {
    pub funcs: FuncList,
    pub val: Val,
}
impl Atom {
    pub fn new<T>(val: T) -> Atom
    where
        Val: From<T>,
    {
        Atom {
            funcs: FuncList::new(),
            val: val.into(),
        }
    }
    pub fn new_express<L, R>(left: L, right: R, op: Operation) -> Atom
    where
        Atom: From<L>,
        Atom: From<R>,
    {
        Val::Express {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op,
        }
        .into()
    }
    fn eval(&self, limit: bool) -> Option<f64> {
        let num = match &self.val {
            Val::Num(n) => Some(*n),
            Val::Express { left, right, op } => {
                op.apply_if_limit(left.eval(limit)?, right.eval(limit)?, limit)
            }
        };
        self.funcs
            .iter()
            .fold(num, |acc, func| func.apply_if_limit(acc?, limit))
    }
    pub fn eval_verbose(&self) -> Option<f64> {
        let num = match &self.val {
            Val::Num(n) => Some(*n),
            Val::Express { left, right, op } => {
                let left = left.eval_verbose()?;
                let right = right.eval_verbose()?;
                let res = op.apply_if_limit(left, right, false);
                println!("{} {} {} = {}", left, op, right, res.unwrap_or(f64::NAN));
                res
            }
        };
        self.funcs.iter().fold(num, |acc, func| {
            func.apply_if_limit(acc?, false).and_then(|res| {
                println!("{}({}) = {}", func, acc?, res);
                Some(res)
            })
        })
    }

    pub fn test(&self, goal: f64) -> bool {
        if !within_error(self.eval(true).unwrap_or(f64::NAN), goal) {
            return false;
        }
        if !self.all_funcs_necessary(goal) {
            return false;
        }
        true
    }
    fn all_funcs_necessary(&self, goal: f64) -> bool {
        self.possible_vals_with_removed_funcs()
            .iter()
            .rev()
            .skip(1) // skip the last one because it's the original atom
            .all(|x| !within_error(*x, goal))
    }
    #[inline(never)]
    fn possible_vals_with_removed_funcs(&self) -> Vec<f64> {
        let possible_num = match &self.val {
            Val::Num(n) => vec![*n],
            Val::Express { left, right, op } => {
                let left = left.possible_vals_with_removed_funcs();
                let right = right.possible_vals_with_removed_funcs();
                left.into_iter()
                    .cartesian_product(right.into_iter())
                    .filter_map(|(l, r)| op.apply_no_limit(l, r))
                    .collect()
            }
        };
        if self.funcs.len() == 0 {
            return possible_num;
        }
        // println!("new start");
        self.funcs
            .iter()
            .group_by(|func| func.clone())
            .into_iter()
            .map(|(func, group)| (0..=group.count()).map(move |i| (func.clone(), i)))
            .multi_cartesian_product()
            .map(|product| {
                // pretty print product
                // println!(
                //     "product: {}",
                //     product
                //         .iter()
                //         .map(|(func, func_repeat)| {
                //             if *func_repeat == 0 {
                //                 "".to_string()
                //             } else {
                //                 let mut s = func.to_string();
                //                 for _ in 1..*func_repeat {
                //                     s.push_str(&format!("{func}"));
                //                 }
                //                 s
                //             }
                //         })
                //         .collect::<String>()
                // );
                possible_num.iter().filter_map(move |num| {
                    product.iter().fold(Some(*num), |num, (func, func_repeat)| {
                        (0..*func_repeat).fold(num, |num, _| func.apply_no_limit(num?))
                    })
                })
            })
            .flatten()
            .collect()
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut end_str = String::new();

        for (i, func) in self.funcs.reverse().iter().enumerate() {
            let outermost = i == 0;
            if func.is_behind() {
                if outermost {
                    end_str.insert_str(0, &format!("{}", func));
                } else {
                    write!(f, "((")?;
                    end_str.insert_str(0, &format!("){})", func));
                }
            } else {
                write!(f, "{}", func)?;
            }
        }
        if let Val::Express { .. } = self.val {
            write!(f, "(")?;
            end_str.insert_str(0, ")");
        }

        match &self.val {
            Val::Num(n) => write!(f, "{}", n)?,
            Val::Express { left, right, op } => {
                if op.is_switched() {
                    write!(f, "{} {} {}", right, op, left)?
                } else {
                    write!(f, "{} {} {}", left, op, right)?
                }
            }
        };

        write!(f, "{}", end_str)
    }
}

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
    fn count_funcs(&self) -> usize {
        self.funcs.len()
            + match &self.val {
                Val::Num(..) => 0,
                Val::Express { left, right, .. } => left.count_funcs() + right.count_funcs(),
            }
    }
    fn eval_with_func_mask(&self, limit: bool, mask: u64) -> Option<f64> {
        fn rec(atom: &Atom, limit: bool, mask: u64, i: &mut usize) -> Option<f64> {
            let mut num = match &atom.val {
                Val::Num(n) => Some(*n),
                Val::Express { left, right, op } => op.apply_if_limit(
                    rec(left, limit, mask, i)?,
                    rec(right, limit, mask, i)?,
                    limit,
                ),
            };
            for func in atom.funcs.iter() {
                // check if mask at i is 1
                if mask & (1 << *i) != 0 {
                    num = func.apply_if_limit(num?, limit);
                }
                *i += 1;
            }
            num
        }
        rec(self, limit, mask, &mut 0)
    }
    pub fn keep_funcs_with_mask(&mut self, mask: u64) {
        fn rec(atom: &mut Atom, mask: u64, i: &mut usize) {
            match &mut atom.val {
                Val::Num(..) => {}
                Val::Express { left, right, .. } => {
                    rec(left, mask, i);
                    rec(right, mask, i);
                }
            }
            let mut new_funcs = FuncList::new();
            for func in atom.funcs.iter() {
                // check if mask at i is 1
                if mask & (1 << *i) != 0 {
                    new_funcs.push(func.clone());
                }
                *i += 1;
            }
            atom.funcs = new_funcs;
        }
        rec(self, mask, &mut 0);
    }
    fn traverse<F>(&self, f: &mut F)
    where
        F: FnMut(&Atom),
    {
        f(self);
        match &self.val {
            Val::Num(..) => {}
            Val::Express { left, right, .. } => {
                left.traverse(f);
                right.traverse(f);
            }
        }
    }
    pub fn score(&self) -> u32 {
        let mut num_count = 0;
        let mut power_count = 0;
        let mut func_count = 0;
        self.traverse(&mut |atom| {
            func_count += atom.funcs.len() as u32;
            match &atom.val {
                Val::Num(..) => num_count += 1,
                Val::Express { op, .. } => power_count += op.score(),
            }
        });
        // extra points for all numbers used
        if num_count == 5 {
            num_count += 1;
        }
        num_count + power_count + func_count
    }
    pub fn simplify(mut self, goal: f64) -> Option<Atom> {
        if !within_error(self.eval(true).unwrap_or(f64::NAN), goal) {
            return None;
        }
        let masks = (0..2u64.pow(self.count_funcs() as u32) - 1)
            .sorted_by(|a, b| a.count_ones().cmp(&b.count_ones()));
        for mask in masks {
            if self
                .eval_with_func_mask(false, mask)
                .map(|res| within_error(res, goal))
                .unwrap_or(false)
            {
                self.keep_funcs_with_mask(mask);
            }
        }
        Some(self)
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

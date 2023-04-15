use itertools::Itertools;

use super::func_list::FuncList;
use super::math::within_error;
use core::panic;
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
    Hole,
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
type AtomStep = (usize, usize);
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
    pub fn new_hole() -> Atom {
        Val::Hole.into()
    }
    fn eval(&self, limit: bool) -> Option<f64> {
        let num = match &self.val {
            Val::Num(n) => Some(*n),
            Val::Express { left, right, op } => {
                op.apply_if_limit(left.eval(limit)?, right.eval(limit)?, limit)
            }
            Val::Hole => panic!("eval with hole"),
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
            Val::Hole => panic!("eval with hole"),
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
            Val::Hole => panic!("eval with hole"),
        };
        if self.funcs.len() == 0 {
            return possible_num;
        }
        self.funcs
            .iter()
            .group_by(|func| func.clone())
            .into_iter()
            .map(|(func, group)| (0..=group.count()).map(move |i| (func.clone(), i)))
            .multi_cartesian_product()
            .map(|product| {
                possible_num.iter().filter_map(move |num| {
                    product.iter().fold(Some(*num), |num, (func, func_repeat)| {
                        (0..*func_repeat).fold(num, |num, _| func.apply_no_limit(num?))
                    })
                })
            })
            .flatten()
            .collect()
    }
    pub fn fill_hole(&mut self, atom: Atom) {
        match &mut self.val {
            Val::Hole => {
                self.val = atom.val;
                let mut new_funcs = FuncList::new();
                for func in atom.funcs.iter() {
                    new_funcs.push(func);
                }
                for func in self.funcs.iter() {
                    new_funcs.push(func);
                }
                self.funcs = new_funcs;
            }
            Val::Express { left, right, .. } => {
                left.fill_hole(atom.clone());
                right.fill_hole(atom);
            }
            Val::Num(_) => {}
        }
    }
    pub fn get_steps_with_eval(&self) -> Vec<(f64, AtomStep)> {
        let mut steps = Vec::new();
        fn rec(atom: &Atom, i: &mut usize, steps: &mut Vec<(f64, AtomStep)>) -> Option<f64> {
            let atom_step = i.clone();
            *i += 1;
            let mut num = match &atom.val {
                Val::Num(n) => Some(*n),
                Val::Express { left, right, op } => op.apply(
                    rec(left, i, steps).expect("fallible atom"),
                    rec(right, i, steps).expect("fallible atom"),
                ),
                Val::Hole => panic!("eval with hole"),
            }?;
            steps.push((num, (atom_step, 0)));
            for (j, func) in atom.funcs.iter().enumerate() {
                num = func.apply(num).expect("fallible atom");
                steps.push((num, (atom_step, j + 1)));
            }
            Some(num)
        }
        rec(self, &mut 0, &mut steps);
        steps
    }
    pub fn split(mut self, step: AtomStep) -> (Atom, Atom) {
        let mut inner_atom = None;
        fn rec(
            atom: &mut Atom,
            i: &mut usize,
            target_step: AtomStep,
            inner_atom: &mut Option<Atom>,
        ) {
            if inner_atom.is_some() {
                return;
            }
            let (atom_step, func_step) = target_step;
            // println!("step {:?}", target_step);
            // println!("i {}", *i);
            // println!("funcs {}", atom.funcs.len());
            // println!("atom: {}", atom);
            if *i == atom_step {
                let mut inner_funcs = FuncList::new();
                for j in 0..func_step {
                    inner_funcs.push(atom.funcs.get(j));
                }
                let mut outer_funcs = FuncList::new();
                for j in func_step..atom.funcs.len() {
                    outer_funcs.push(atom.funcs.get(j));
                }

                let inner_val = atom.val.clone();
                *inner_atom = Some(Atom {
                    val: inner_val,
                    funcs: inner_funcs,
                });

                atom.funcs = outer_funcs;
                atom.val = Val::Hole;
            } else {
                *i += 1;
                match &mut atom.val {
                    Val::Num(_) => {}
                    Val::Express { left, right, .. } => {
                        rec(left, i, target_step, inner_atom);
                        rec(right, i, target_step, inner_atom);
                    }
                    Val::Hole => panic!("eval with hole"),
                }
            }
        }
        rec(&mut self, &mut 0, step, &mut inner_atom);
        let inner_atom = inner_atom.expect("no atom at step");
        (self, inner_atom)
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
            Val::Hole => write!(f, "[hole]")?,
        };

        write!(f, "{}", end_str)
    }
}

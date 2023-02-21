use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use crate::finder::func::Func;
use crate::finder::operation::Operation;

pub struct FuncAtom {
    pub atom: Atom,
    pub funcs: Vec<Func>,
    pub distribution: Vec<usize>,
}
impl FuncAtom {
    pub fn new(atom: Atom, funcs: &[Func], distribution: &[usize]) -> FuncAtom {
        FuncAtom {
            atom,
            funcs: funcs.to_vec(),
            distribution: distribution.to_vec(),
        }
    }
    pub fn eval_verbose(&self) {
        self.atom.eval_verbose(&self.funcs, &self.distribution);
    }
}
impl Display for FuncAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        self.atom
            .fmt_with_funcs(f, &self.funcs, &self.distribution, &mut i)
    }
}
#[derive(Debug, Clone)]
pub enum Atom {
    Number(f64),
    Express {
        left: Box<Atom>,
        right: Box<Atom>,
        op: Operation,
    },
}

impl From<f64> for Atom {
    fn from(n: f64) -> Self {
        Atom::Number(n).into()
    }
}
impl From<&f64> for Atom {
    fn from(n: &f64) -> Self {
        Atom::Number(*n).into()
    }
}
enum AtomOpFunc<'a> {
    Atom(&'a Atom),
    Op(&'a Operation),
    Func(&'a Func),
}
impl Atom {
    pub fn new_express<L, R>(left: L, right: R, op: Operation) -> Atom
    where
        Atom: From<L>,
        Atom: From<R>,
    {
        Atom::Express {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op,
        }
        .into()
    }

    fn distribute_funcs<'a>(
        funcs: &'a [Func],
        distribution: &'a [usize],
        index: usize,
    ) -> impl Iterator<Item = &'a Func> + DoubleEndedIterator {
        distribution
            .iter()
            .enumerate()
            .filter(move |(_, atom_index)| **atom_index == index)
            .map(move |(func_index, _)| &funcs[func_index])
    }

    fn traverse<F>(&self, f: &mut F)
    where
        F: FnMut(&Atom),
    {
        f(self);
        match self {
            Atom::Number(..) => {}
            Atom::Express { left, right, .. } => {
                left.traverse(f);
                right.traverse(f);
            }
        }
    }

    pub fn score(&self) -> u32 {
        let mut num_count = 0;
        let mut power_count = 0;
        // use traverse
        self.traverse(&mut |atom| match atom {
            Atom::Number(..) => num_count += 1,
            Atom::Express { op, .. } => match op {
                Operation::Power => power_count += 1,
                Operation::Root => power_count += 1,
                _ => (),
            },
        });
        // extra points for all numbers used
        if num_count == 5 {
            num_count += 1;
        }
        num_count + power_count
    }
    pub fn count_atoms(&self) -> u32 {
        let mut count = 0;
        self.traverse(&mut |_| count += 1);
        count
    }

    fn fmt_with_funcs(
        &self,
        f: &mut Formatter<'_>,
        funcs: &[Func],
        distribution: &[usize],
        i: &mut usize,
    ) -> std::fmt::Result {
        let mut end_str = String::new();
        for func in Atom::distribute_funcs(funcs, distribution, *i).rev() {
            if !func.is_behind() {
                write!(f, "{}", func)?;
            }
            write!(f, "(")?;

            end_str.push(')');
            if func.is_behind() {
                end_str.push_str(&format!("{}", func));
            }
        }

        *i += 1;

        match self {
            Atom::Number(n) => write!(f, "{}", n)?,
            Atom::Express { left, right, op } => {
                write!(f, "(")?;
                left.fmt_with_funcs(f, funcs, distribution, i)?;
                write!(f, " {} ", op)?;
                right.fmt_with_funcs(f, funcs, distribution, i)?;
                write!(f, ")")?;
            }
        };

        write!(f, "{}", end_str)
    }
    pub fn eval(&self) -> Option<f64> {
        self.eval_with_funcs(&[], &[])
    }
    pub fn eval_with_funcs(&self, funcs: &[Func], distribution: &[usize]) -> Option<f64> {
        let mut i = 0;
        self.eval_rec(funcs, distribution, &mut i)
    }
    fn eval_rec(&self, funcs: &[Func], distribution: &[usize], i: &mut usize) -> Option<f64> {
        let distributed = Atom::distribute_funcs(funcs, distribution, *i);
        *i += 1;
        let num = match self {
            Atom::Express { left, right, op } => op.apply(
                left.eval_rec(funcs, distribution, i)?,
                right.eval_rec(funcs, distribution, i)?,
            ),
            Atom::Number(n) => Some(*n),
        };
        distributed.fold(num, |acc, func| func.apply(acc?))
    }
    fn eval_loop(&self, funcs: &[Func], distribution: &[usize], i: &mut usize) -> Option<f64> {
        let mut queue: VecDeque<AtomOpFunc> = VecDeque::new();
        queue.push_back(AtomOpFunc::Atom(self));
        let mut stack: VecDeque<f64> = VecDeque::new();
        while let Some(atom) = queue.pop_front() {
            match atom {
                AtomOpFunc::Atom(atom) => {
                    let distributed = Atom::distribute_funcs(funcs, distribution, *i);
                    *i += 1;
                    for func in distributed.rev() {
                        queue.push_front(AtomOpFunc::Func(func));
                    }
                    match atom {
                        Atom::Express { left, right, op } => {
                            queue.push_front(AtomOpFunc::Op(op));
                            queue.push_front(AtomOpFunc::Atom(right));
                            queue.push_front(AtomOpFunc::Atom(left));
                        }
                        Atom::Number(n) => stack.push_front(*n),
                    }
                }
                AtomOpFunc::Op(op) => {
                    let right = stack.pop_front()?;
                    let left = stack.pop_front()?;
                    stack.push_front(op.apply(left, right)?);
                }
                AtomOpFunc::Func(func) => {
                    let num = stack.pop_front()?;
                    stack.push_front(func.apply(num)?);
                }
            }
        }
        stack.pop_back()
    }

    fn eval_verbose(&self, funcs: &[Func], distribution: &[usize]) -> Option<f64> {
        let mut i = 0;
        self.eval_verbose_rec(funcs, distribution, &mut i)
    }
    fn eval_verbose_rec(
        &self,
        funcs: &[Func],
        distribution: &[usize],
        i: &mut usize,
    ) -> Option<f64> {
        let distributed = Atom::distribute_funcs(funcs, distribution, *i);
        *i += 1;
        let num = match self {
            Atom::Express { left, right, op } => {
                match (
                    left.eval_verbose_rec(funcs, distribution, i),
                    right.eval_verbose_rec(funcs, distribution, i),
                ) {
                    (Some(left), Some(right)) => {
                        let ret = op.apply(left, right);
                        println!("{} {} {} = {}", left, op, right, ret.unwrap());
                        ret
                    }
                    _ => None,
                }
            }
            Atom::Number(n) => Some(*n),
        };
        distributed.fold(num, |num, func| {
            num.and_then(|n| {
                let out = func.apply(n);
                println!("{}({}) = {}", func, n, out.unwrap());
                out
            })
        })
    }
}
impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Number(n) => write!(f, "{}", n),
            Atom::Express { left, right, op } => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

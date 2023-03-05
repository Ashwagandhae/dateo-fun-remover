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

    #[inline]
    pub fn distribute_funcs<'a>(
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
    #[inline(never)]
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
            if func.is_behind() {
                write!(f, "(")?;
                end_str.push(')');
                end_str.push_str(&format!("{}", func));
            } else {
                write!(f, "{}", func)?;
                write!(f, "(")?;
                end_str.push(')');
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

    pub fn eval_verbose(&self, funcs: &[Func], distribution: &[usize]) -> Option<f64> {
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
                        println!("{} {} {} = {}", left, op, right, ret.unwrap_or(f64::NAN));
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
                println!("{}({}) = {}", func, n, out.unwrap_or(f64::NAN));
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

use std::fmt::{Display, Formatter};

use crate::finder::function::Function;
use crate::finder::operation::Operation;

#[derive(Debug, Clone)]
pub struct Atom {
    functions: Vec<Function>,
    val: AtomVal,
}
#[derive(Debug, Clone)]
enum AtomVal {
    Number(f64),
    Express {
        left: Box<Atom>,
        right: Box<Atom>,
        op: Operation,
    },
}
impl From<AtomVal> for Atom {
    fn from(val: AtomVal) -> Self {
        Atom {
            functions: Vec::new(),
            val,
        }
    }
}
impl From<f64> for Atom {
    fn from(n: f64) -> Self {
        AtomVal::Number(n).into()
    }
}
impl From<&f64> for Atom {
    fn from(n: &f64) -> Self {
        AtomVal::Number(*n).into()
    }
}
impl Atom {
    pub fn new_express<L, R>(left: L, right: R, op: Operation) -> Atom
    where
        Atom: From<L>,
        Atom: From<R>,
    {
        AtomVal::Express {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op,
        }
        .into()
    }
    pub fn eval(&self) -> Option<f64> {
        let mut num = match &self.val {
            AtomVal::Express { left, right, op } => match (left.eval(), right.eval()) {
                (Some(left), Some(right)) => {
                    let ret = op.apply(left, right);
                    ret
                }
                _ => None,
            },
            AtomVal::Number(n) => Some(*n),
        };
        for func in &self.functions {
            num = num.and_then(|n| func.apply(n));
        }
        num
    }
    pub fn eval_verbose(&self) -> Option<f64> {
        let mut num = match &self.val {
            AtomVal::Express { left, right, op } => {
                match (left.eval_verbose(), right.eval_verbose()) {
                    (Some(left), Some(right)) => {
                        let ret = op.apply(left, right);
                        println!("{} {} {} = {}", left, op, right, ret.unwrap());
                        ret
                    }
                    _ => None,
                }
            }
            AtomVal::Number(n) => Some(*n),
        };
        for func in &self.functions {
            let old_num = num.clone();
            num = num.and_then(|n| func.apply(n));
            println!("{}({}) = {}", func, old_num.unwrap(), num.unwrap());
        }
        num
    }
    pub fn score(&self) -> u32 {
        let mut num_count = 0;
        let mut power_count = 0;
        let mut func_count = 0;
        self.score_rec(&mut num_count, &mut power_count, &mut func_count);
        // extra points for all numbers used
        if num_count == 5 {
            num_count += 1;
        }
        power_count + num_count + func_count
    }
    fn score_rec(&self, num_count: &mut u32, power_count: &mut u32, func_count: &mut u32) {
        *func_count += self.functions.len() as u32;
        match &self.val {
            AtomVal::Number(..) => *num_count += 1,
            AtomVal::Express { left, right, op } => {
                match op {
                    Operation::Power => *power_count += 1,
                    Operation::Root => *power_count += 1,
                    _ => (),
                }
                left.score_rec(num_count, power_count, func_count);
                right.score_rec(num_count, power_count, func_count);
            }
        }
    }
    pub fn count_atoms(&self) -> u32 {
        match &self.val {
            AtomVal::Number(..) => 1,
            AtomVal::Express { left, right, .. } => left.count_atoms() + right.count_atoms() + 1,
        }
    }
    // pub fn should_keep(&self) -> bool {
    //     self.should_keep_rec().1
    // }
    // // remove atoms with wrong order mult and add ops
    // fn should_keep_rec(&self) -> (Option<f64>, bool) {
    //     match &self.val {
    //         AtomVal::Express { left, right, op } => {
    //             match (left.should_keep_rec(), right.should_keep_rec()) {
    //                 // if either side is false, then the whole thing is false
    //                 ((_, left_keep), (_, right_keep)) if !left_keep || !right_keep => (None, false),
    //                 ((Some(left), _), (Some(right), _)) => match op {
    //                     Operation::Add | Operation::Multiply => {
    //                         // this is arbitrary, we just need to remove half of the mult and add ops
    //                         // because they are commutative, so its unnecessary to keep both
    //                         // we don't handle the case where left == right
    //                         if left >= right {
    //                             (None, false)
    //                         } else {
    //                             (op.apply(left, right), true)
    //                         }
    //                     }
    //                     _ => (op.apply(left, right), true),
    //                 },
    //                 _ => (None, true),
    //             }
    //         }
    //         AtomVal::Number(n) => (Some(*n), true),
    //     }
    // }
    pub fn add_funcs(&mut self, funcs: &Vec<Function>, distribution: &Vec<usize>) {
        let mut i = 0;
        self.add_funcs_rec(funcs, distribution, &mut i);
    }
    fn add_funcs_rec(&mut self, funcs: &Vec<Function>, distribution: &Vec<usize>, i: &mut usize) {
        self.functions = distribution
            .iter()
            .enumerate()
            .filter(|(_, atom_index)| *atom_index == i)
            .map(|(func_index, _)| funcs[func_index].clone())
            .collect();
        *i += 1;
        match &mut self.val {
            AtomVal::Express { left, right, .. } => {
                left.add_funcs_rec(funcs, distribution, i);
                right.add_funcs_rec(funcs, distribution, i);
            }
            AtomVal::Number(..) => {}
        }
    }
}
impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for func in self.functions.iter().rev() {
            if func.is_behind() {
                write!(f, "(")?;
            } else {
                write!(f, "{}(", func)?;
            }
        }
        match &self.val {
            AtomVal::Number(n) => write!(f, "{}", n),
            AtomVal::Express { left, right, op } => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
        .and_then(|_| {
            for func in self.functions.iter().rev() {
                if func.is_behind() {
                    write!(f, "){}", func)?;
                } else {
                    write!(f, ")")?;
                }
            }
            Ok(())
        })
    }
}

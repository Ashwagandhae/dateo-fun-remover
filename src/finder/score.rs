use super::func::Func;
use super::func_list::FuncList;
use super::operation::Operation;
use std::fmt::Display;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct PendingFuncScore {
    pub func_list: FuncList,
}
#[derive(Clone, Copy, Debug)]
pub struct Score {
    pub nums: u8,
    pub pending_funcs: PendingFuncScore,
    pub funcs: u8,
    pub power_ops: u8,
    pub root_ops: u8,
}
impl Score {
    // pub fn empty() -> Self {
    //     Self {
    //         nums: 0,
    //         square_root_funcs: 0,
    //         factorial_funcs: 0,
    //         summation_funcs: 0,
    //         power_ops: 0,
    //         root_ops: 0,
    //     }
    // }
    pub fn from_nums(nums: usize) -> Self {
        Self {
            nums: nums as u8,
            pending_funcs: PendingFuncScore {
                func_list: FuncList::new(),
            },
            funcs: 0,
            power_ops: 0,
            root_ops: 0,
        }
    }
    pub fn resolve(mut self) -> Self {
        const MAX_CONSECUTIVE_SQAURE_ROOT: u8 = 4;
        // self.funcs += self.pending_funcs.square_root_funcs;
        // // .min(MAX_SQAURE_ROOT_FUNCS);
        // self.funcs += self.pending_funcs.factorial_funcs;
        // self.funcs += self.pending_funcs.summation_funcs;
        let mut current_consecutive_square_root = 0;
        for func in self.pending_funcs.func_list.iter() {
            match func {
                Func::SquareRoot => {
                    current_consecutive_square_root += 1;
                    if current_consecutive_square_root <= MAX_CONSECUTIVE_SQAURE_ROOT {
                        self.funcs += 1;
                    }
                }
                Func::Factorial => {
                    current_consecutive_square_root = 0;
                    self.funcs += 1;
                }
                Func::Summation => {
                    current_consecutive_square_root = 0;
                    self.funcs += 1;
                }
            }
        }

        self.pending_funcs = PendingFuncScore {
            func_list: FuncList::new(),
        };
        self
    }
    pub fn score(&self) -> u8 {
        if self.pending_funcs.func_list.len() > 0 {
            panic!("cannot score unresolved Score");
        }
        let base_score = self.nums + self.funcs + self.power_ops + self.root_ops;

        let num_bonus = if self.nums == 5 { 1 } else { 0 };

        base_score + num_bonus
    }
    // pub fn score(&self) -> u8 {
    //     let base_score = self.nums
    //         + self.square_root_funcs
    //         + self.factorial_funcs
    //         + self.summation_funcs
    //         + self.power_ops
    //         + self.root_ops;
    //     let num_bonus = if self.nums == 5 { 1 } else { 0 };
    //     let func_bonus = self
    //         .square_root_funcs
    //         .min(self.factorial_funcs.min(self.summation_funcs))
    //         * 4;

    //     base_score + num_bonus + func_bonus
    // }
    // pub fn score(&self) -> u8 {
    //     let mut square_score = 1;
    //     let mut factorial_score = 1;
    //     let mut summation_score = 1;
    //     if self.square_root_funcs > 3 {
    //         factorial_score += 1;
    //         summation_score += 1;
    //     }
    //     if self.factorial_funcs > 3 {
    //         square_score += 1;
    //         summation_score += 1;
    //     }
    //     if self.summation_funcs > 3 {
    //         square_score += 1;
    //         factorial_score += 1;
    //     }
    //     let base_score = self.nums
    //         + self.square_root_funcs * square_score
    //         + self.factorial_funcs * factorial_score
    //         + self.summation_funcs * summation_score
    //         + self.power_ops
    //         + self.root_ops;
    //     let num_bonus = if self.nums == 5 { 1 } else { 0 };

    //     base_score + num_bonus
    // }

    pub fn add_funcs_list(mut self, funcs: FuncList) -> Self {
        self.pending_funcs.func_list = self.pending_funcs.func_list.join(funcs);
        self
    }
    // pub fn add_func(mut self, func: Func) -> Self {
    //     match func {
    //         Func::SquareRoot => self.square_root_funcs += 1,
    //         Func::Factorial => self.factorial_funcs += 1,
    //         Func::Summation => self.summation_funcs += 1,
    //     }
    //     self
    // }
    pub fn add_op(mut self, op: Operation) -> Self {
        match op {
            Operation::Power => self.power_ops += 1,
            Operation::PowerSwitch => self.power_ops += 1,
            Operation::Root => self.root_ops += 1,
            Operation::RootSwitch => self.root_ops += 1,
            _ => {}
        }
        self
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (n: {}, o: {}, f: {})",
            self.score(),
            self.nums,
            self.power_ops + self.root_ops,
            self.funcs,
        )
    }
}

impl Add for Score {
    type Output = Score;
    fn add(self, other: Score) -> Self::Output {
        Score {
            nums: self.nums + other.nums,
            pending_funcs: PendingFuncScore {
                func_list: self
                    .pending_funcs
                    .func_list
                    .join(other.pending_funcs.func_list),
            },
            funcs: self.funcs + other.funcs,
            power_ops: self.power_ops + other.power_ops,
            root_ops: self.root_ops + other.root_ops,
        }
    }
}

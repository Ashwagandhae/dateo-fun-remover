pub mod atom;
pub mod func;
pub mod func_list;
pub mod joiner;
pub mod math;
pub mod operation;
pub mod score;
pub mod tree;
pub mod tree_shapes;

use func::Func;
use itertools::Itertools;
use joiner::get_joiners;
use joiner::{AtomFilter, Memo};
use score::Score;

use crate::finder::atom::Atom;
use crate::finder::operation::Operation;

const SQUARES_DEPTH: usize = 4;
const DEPTH: usize = 5;

pub fn solve(nums: &[f64], goal: f64, callback: impl Fn(Score, &Atom)) {
    let mut best_score = 0;
    println!("solving squares");
    solve_squares(nums, goal, &callback, &mut best_score);
    println!("solving other");
    solve_all(nums, goal, &callback, &mut best_score);
}

fn solve_all(nums: &[f64], goal: f64, callback: impl Fn(Score, &Atom), best_score: &mut u8) {
    let mut memo = Memo::new();
    for num_count in (1..=5).rev() {
        let joiners = get_joiners(num_count);
        for mut joiner in joiners {
            for (score, atom) in joiner.solve(
                &nums,
                goal,
                DEPTH,
                AtomFilter::MinScore(*best_score),
                &mut memo,
            ) {
                *best_score = score.score();
                callback(score, &atom);
            }
        }
    }
}

fn solve_squares(nums: &[f64], goal: f64, callback: impl Fn(Score, &Atom), best_score: &mut u8) {
    let mut memo = Memo::new();
    let combinations = (1..=2)
        .rev()
        .flat_map(|split| combinations_when_split(&nums, split));
    for (goal_nums, power_nums) in combinations {
        for (score, atom) in solve_square(
            &goal_nums,
            &power_nums,
            goal,
            AtomFilter::MinScore(*best_score),
            &mut memo,
        ) {
            *best_score = score.score();
            callback(score, &atom);
        }
    }
}

const POWER_OF_2: [f64; 30] = [
    1., 2., 4., 8., 16., 32., 64., 128., 256., 512., 1024., 2048., 4096., 8192., 16384., 32768.,
    65536., 131072., 262144., 524288., 1048576., 2097152., 4194304., 8388608., 16777216.,
    33554432., 67108864., 134217728., 268435456., 536870912.,
];

fn solve_square<'a>(
    goal_nums: &[f64],
    power_nums: &'a [f64],
    goal: f64,
    mut atom_filter: AtomFilter,
    memo: &'a mut Memo,
) -> impl Iterator<Item = (Score, Atom)> + 'a {
    let goal_joiners = get_joiners(goal_nums.len());
    let mut power_joiners = get_joiners(power_nums.len());

    let goal_solutions = goal_joiners
        .into_iter()
        .flat_map(|mut joiner| {
            joiner
                .solve(goal_nums, goal, SQUARES_DEPTH, AtomFilter::None, memo)
                .into_iter()
                .collect_vec()
        })
        .collect_vec();
    goal_solutions
        .into_iter()
        .flat_map(move |(_goal_score, goal_atom)| {
            let goal_atom_steps = goal_atom.get_steps_with_eval();
            let (inner_goal, max_step) = goal_atom_steps
                .into_iter()
                .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
                .unwrap();
            let (goal_atom_outer, goal_atom_inner) = goal_atom.split(max_step);
            let (max_inner_sqrt, n) = max_func_applications(inner_goal, Func::SquareRoot, false);
            let max_outer_sqrt =
                max_func_applications(n, Func::SquareRoot, true).0 - max_inner_sqrt;
            let max_sqrt = (max_inner_sqrt + max_outer_sqrt).min(30);
            (1..max_sqrt)
                .rev()
                .flat_map(|sqrt_count| {
                    let power_of_2 = POWER_OF_2[sqrt_count];
                    // let inner_sqrt = max_inner_sqrt.min(sqrt_count);
                    // let outer_sqrt = sqrt_count - inner_sqrt;
                    let outer_sqrt = max_outer_sqrt.min(sqrt_count);
                    let inner_sqrt = sqrt_count - outer_sqrt;
                    let joiner_solutions = power_joiners
                        .iter_mut()
                        .flat_map(|joiner| {
                            joiner
                                .solve(
                                    power_nums,
                                    power_of_2,
                                    SQUARES_DEPTH,
                                    AtomFilter::None,
                                    memo,
                                )
                                .into_iter()
                                .collect_vec()
                        })
                        .take(3)
                        .collect_vec();
                    joiner_solutions
                        .iter()
                        .cloned()
                        .filter_map(|(_power_score, power_atom)| {
                            let mut goal_atom_inner = goal_atom_inner.clone();
                            for _ in 0..inner_sqrt {
                                goal_atom_inner.funcs.push(Func::SquareRoot);
                            }
                            let mut inner_goal_atom =
                                Atom::new_express(goal_atom_inner, power_atom, Operation::Power);
                            for _ in 0..outer_sqrt {
                                inner_goal_atom.funcs.push(Func::SquareRoot);
                            }
                            let mut atom = goal_atom_outer.clone();
                            atom.fill_hole(inner_goal_atom);

                            let score = atom.get_score();
                            if let AtomFilter::MinScore(min_score) = atom_filter {
                                if score.score() <= min_score {
                                    return None;
                                }
                            }
                            if !atom.test(goal) {
                                return None;
                            }
                            if let AtomFilter::MinScore(min_score) = &mut atom_filter {
                                *min_score = score.score();
                            }
                            Some((score, atom))
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
}

pub fn max_func_applications(mut num: f64, func: Func, rev: bool) -> (usize, f64) {
    let mut count = 0;
    while count < 100 {
        let new_num = func.apply_rev_if(num, rev);
        match new_num {
            Some(new_num) => num = new_num,
            None => return (count, num),
        }
        count += 1;
    }
    (count, num)
}

pub fn combinations_when_split<'a>(
    nums: &'a [f64],
    split: usize,
) -> impl Iterator<Item = (Vec<f64>, Vec<f64>)> + 'a {
    fn rec(len: usize, split: usize) -> impl Iterator<Item = Vec<usize>> {
        if split == 1 {
            return (0..len).map(|x| vec![x]).collect_vec().into_iter();
        }
        let mut all_chosen: Vec<Vec<usize>> = Vec::new();
        for i in 0..(len - split + 1) {
            all_chosen.extend(rec(len - i - 1, split - 1).map(|x| {
                std::iter::once(i)
                    .chain(x.iter().map(|y| y + i + 1))
                    .collect_vec()
            }));
        }
        all_chosen.into_iter()
    }
    rec(nums.len(), split).map(move |chosen| {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            if chosen.contains(&i) {
                left.push(*num);
            } else {
                right.push(*num);
            }
        }
        (left, right)
    })
}

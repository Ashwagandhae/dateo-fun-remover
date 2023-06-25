use itertools::iproduct;
use itertools::repeat_n;
use itertools::Itertools;
use math::within_error;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use strum::IntoEnumIterator;

pub mod operation;
use operation::Operation;
pub mod func;
use func::Func;
pub mod atom;
use atom::{Atom, FuncAtom};
pub mod atom_store;
use atom_store::{AtomGroup, AtomStore};
pub mod math;
use ordered_float::OrderedFloat;

pub mod codon;

const PARA: bool = true;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Used(u8);

impl Used {
    fn new() -> Used {
        Used(0)
    }
    fn new_set(index: usize) -> Used {
        Used(1 << index)
    }
    fn set(&mut self, index: usize) {
        self.0 |= 1 << index;
    }
    fn get(&self, index: usize) -> bool {
        self.0 & (1 << index) != 0
    }

    fn clone_set(&self, index: usize) -> Used {
        let mut clone = self.clone();
        clone.set(index);
        clone
    }
    fn count(&self) -> usize {
        self.0.count_ones() as usize
    }
    fn overlap(&self, other: &Used) -> bool {
        self.0 & other.0 != 0
    }
}
pub fn solution_with_least_funcs(
    goal_paths: &GoalPaths,
    atom: &Atom,
    used: &Used,
    atom_group: &AtomGroup,
    codon_index: usize,
    codon_count: usize,
    funcs: &Vec<Func>,
    distribution: &Vec<usize>,
) -> Option<(FuncAtom, u32)> {
    // fast eval to check if path to goal exists
    let val = atom_group.eval_with_funcs(codon_index, codon_count, funcs, distribution, true)?;
    let path = goal_paths.get_path(val, &used)?;

    // add the funcs from the path to the funcs we already have
    let min_funcs = funcs.len() - path.non_func_score_delta(used) as usize;
    let (funcs, distribution, atom) = path.edit(funcs, distribution, atom);
    // create a bit mask for all possible combinations of funcs
    let bit_mask_range = 2u64.pow(distribution.len() as u32);
    for bit_mask in 0..bit_mask_range {
        let mut new_funcs = Vec::new();
        let mut new_distribution = Vec::new();
        for (i, func_index) in distribution.iter().enumerate() {
            if bit_mask & (1 << i) != 0 {
                new_funcs.push(funcs[i].clone());
                new_distribution.push(*func_index);
            }
        }
        let no_limit_success = atom
            .eval_with_funcs(&new_funcs, &new_distribution, false)
            .map(|test| within_error(test, goal_paths.goal))
            .unwrap_or(false);
        if !no_limit_success {
            continue;
        }
        if new_funcs.len() < min_funcs {
            return None;
        }
        // we know this will be the best solution, because we're starting from all zeros
        // in bit_mask, so we'll have the most funcs possible

        // check if solution works with limit
        let success = atom
            .eval_with_funcs(&new_funcs, &new_distribution, true)
            .map(|test| within_error(test, goal_paths.goal))
            .unwrap_or(false);
        if success {
            return Some((
                FuncAtom::new(atom.clone(), &new_funcs, &new_distribution),
                (new_funcs.len() - min_funcs) as u32,
            ));
        }
    }
    None
}

pub fn get_solution_in_group(
    func_count: &u32,
    goal_paths: &GoalPaths,
    atom_group: &AtomGroup,
) -> Option<(FuncAtom, u32)> {
    fn find_map(
        func_count: &u32,
        goal_paths: &GoalPaths,
        atom_group: &AtomGroup,
        ((atom, used), (codon_index, codon_count)): (&(Atom, Used), &(usize, usize)),
    ) -> Option<(FuncAtom, u32)> {
        Func::iter()
            .combinations_with_replacement(*func_count as usize)
            .find_map(|funcs| {
                (0..atom.count_func_atoms() as usize)
                    .combinations_with_replacement(*func_count as usize)
                    .find_map(|distribution| {
                        solution_with_least_funcs(
                            goal_paths,
                            atom,
                            used,
                            atom_group,
                            *codon_index,
                            *codon_count,
                            &funcs,
                            &distribution,
                        )
                    })
            })
    }
    if PARA {
        atom_group
            .par_iter()
            .find_map_any(|atom_info| find_map(func_count, goal_paths, atom_group, atom_info))
    } else {
        atom_group
            .iter()
            .find_map(|atom_info| find_map(func_count, goal_paths, atom_group, atom_info))
    }
}
pub fn get_solution_with_score(
    min_score: u32,
    goal_paths: &GoalPaths,
    store: &AtomStore,
) -> Option<(FuncAtom, u32)> {
    store
        .iter()
        .filter(|(base_score, _)| **base_score <= min_score)
        .find_map(|(base_score, atom_group)| {
            get_solution_in_group(&(min_score - base_score), goal_paths, atom_group)
        })
}

pub fn create_atoms(nums: &[f64]) -> Vec<(Atom, Used)> {
    fn rec(
        nums: &[f64],
        used: Used,
        min_unused: usize,
        memo: &mut HashMap<(Used, usize), Vec<(Atom, Used)>>,
    ) -> Vec<(Atom, Used)> {
        // try getting from memo
        if let Some(atoms) = memo.get(&(used, min_unused)) {
            return atoms.clone();
        }
        let mut atoms = Vec::new();
        // get nums that aren't used
        let index_nums: Vec<(usize, f64)> = nums
            .iter()
            .enumerate()
            .filter(|(i, _)| !used.get(*i))
            .map(|(i, n)| (i, *n))
            .collect();

        let available_count = nums.len() - used.count() - min_unused;
        // num atoms
        if available_count >= 1 {
            for (i, num) in index_nums.iter() {
                atoms.push((num.into(), used.clone_set(*i)));
            }
        }
        // express atoms
        // need at least 2 unused nums to make an express atom
        if available_count >= 2 {
            for (left, left_used) in rec(nums, used, min_unused + 1, memo) {
                for (right, right_used) in rec(nums, left_used, min_unused, memo) {
                    for op in Operation::iter() {
                        atoms.push((
                            Atom::new_express(left.clone(), right.clone(), op.clone()),
                            right_used,
                        ));
                        if !op.is_commutative() {
                            atoms.push((
                                Atom::new_express(right.clone(), left.clone(), op.clone()),
                                right_used,
                            ));
                        }
                    }
                }
            }
        }
        if used.count() != 0 || min_unused != 0 {
            memo.insert((used, min_unused), atoms.clone());
        }
        atoms
    }
    let mut memo = HashMap::new();
    let ret = rec(nums, Used::new(), 1, &mut memo)
        .into_iter()
        .filter(|(atom, _)| atom.eval_possible())
        .collect();
    ret
}

pub fn create_atom_store(nums: &[f64]) -> AtomStore {
    AtomStore::new(create_atoms(nums))
}

pub fn create_goal_paths(goal: f64, nums: &[f64]) -> GoalPaths {
    GoalPaths::new(goal, nums)
}

#[derive(Clone, Debug)]
enum OperationSide {
    Left,
    Right,
}
#[derive(Clone, Debug)]
enum GoalPath {
    Single {
        outer_funcs: Vec<Func>,
    },
    Double {
        outer_funcs: Vec<Func>,
        num_funcs: Vec<Func>,
        op: Operation,
        side: OperationSide,
        used: Used,
        num: f64,
    },
}
impl GoalPath {
    fn new_double(
        outer_funcs: Vec<Func>,
        num_funcs: Vec<Func>,
        op: Operation,
        side: OperationSide,
        used: Used,
        num: f64,
    ) -> GoalPath {
        GoalPath::Double {
            outer_funcs,
            num_funcs,
            op,
            side,
            used,
            num,
        }
    }
    fn new_single(outer_funcs: Vec<Func>) -> GoalPath {
        GoalPath::Single { outer_funcs }
    }
    fn new_empty() -> GoalPath {
        GoalPath::Single {
            outer_funcs: vec![],
        }
    }
    fn edit(
        &self,
        funcs: &[Func],
        distribution: &[usize],
        atom: &Atom,
    ) -> (Vec<Func>, Vec<usize>, Atom) {
        match self {
            GoalPath::Single { outer_funcs } => {
                let funcs = outer_funcs
                    .iter()
                    .chain(funcs.iter())
                    .cloned()
                    .collect::<Vec<_>>();
                let distribution = repeat_n(0usize, outer_funcs.len())
                    .chain(distribution.iter().cloned())
                    .collect::<Vec<_>>();
                (funcs, distribution, atom.clone())
            }
            GoalPath::Double {
                outer_funcs,
                num_funcs,
                op,
                side,
                num,
                ..
            } => {
                match side {
                    OperationSide::Left => {
                        let new_atom =
                            Atom::new_express::<Atom, &f64>(atom.clone(), num.into(), op.clone());
                        // outer_funcs at start
                        // atom funcs in middle
                        // num_funcs at end
                        let funcs = outer_funcs
                            .iter()
                            .chain(funcs.iter())
                            .chain(num_funcs.iter())
                            .cloned()
                            .collect::<Vec<_>>();
                        // we need to get the end_i for num_funcs distribution
                        let end_i = new_atom.count_func_atoms() as usize - 1;
                        let distribution = repeat_n(0usize, outer_funcs.len())
                            .chain(distribution.iter().map(|i| i + 1))
                            .chain(repeat_n(end_i, num_funcs.len()))
                            .collect::<Vec<_>>();
                        (funcs, distribution, new_atom)
                    }
                    OperationSide::Right => {
                        let new_atom =
                            Atom::new_express::<&f64, Atom>(num.into(), atom.clone(), op.clone());
                        // outer_funcs at start
                        // num_funcs in middle
                        // atom funcs at end
                        let funcs = outer_funcs
                            .iter()
                            .chain(num_funcs.iter())
                            .chain(funcs.iter())
                            .cloned()
                            .collect::<Vec<_>>();
                        // we know i will be 1 for num_funcs distribution
                        let distribution = repeat_n(0usize, outer_funcs.len())
                            .chain(repeat_n(1usize, num_funcs.len()))
                            .chain(distribution.iter().map(|i| i + 2))
                            .collect::<Vec<_>>();
                        (funcs, distribution, new_atom)
                    }
                }
            }
        }
    }
    fn non_func_score_delta(&self, used: &Used) -> u32 {
        match self {
            GoalPath::Single { .. } => 0,
            GoalPath::Double { op, .. } => {
                (if used.count() == 4 { 1 } else { 0 }
                    + match op {
                        Operation::Power | Operation::Root => 1,
                        _ => 0,
                    }
                    + 1)
            }
        }
    }
    fn guess_score_delta(&self) -> u32 {
        match self {
            GoalPath::Single { outer_funcs } => outer_funcs.len() as u32,
            GoalPath::Double {
                outer_funcs,
                num_funcs,
                op,
                ..
            } => {
                outer_funcs.len() as u32
                    + num_funcs.len() as u32
                    + match op {
                        Operation::Power | Operation::Root => 1,
                        _ => 0,
                    }
                    + 1 // assume that there are 4 numbers
            }
        }
    }
}
impl Display for GoalPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GoalPath::Single { outer_funcs } => {
                write!(f, "{}", outer_funcs.iter().rev().join(" "))
            }
            GoalPath::Double {
                outer_funcs,
                num_funcs,
                op,
                side,
                num,
                ..
            } => {
                let num_funcs = num_funcs.iter().join(" ");
                let outer_funcs = outer_funcs.iter().rev().join(" ");
                match side {
                    OperationSide::Left => write!(
                        f,
                        "{} {} {}",
                        format!("{} {}", num_funcs, num),
                        op,
                        outer_funcs,
                    ),
                    OperationSide::Right => write!(
                        f,
                        "{} {} {}",
                        outer_funcs,
                        op,
                        format!("{} {}", num_funcs, num),
                    ),
                }
            }
        }
    }
}
fn expand_funcs(start: f64, reverse: bool) -> Vec<(f64, Vec<Func>)> {
    let mut highest_level_paths: Vec<(f64, Vec<Func>)> = vec![(start, vec![])];
    let mut paths = highest_level_paths.clone();
    while highest_level_paths.len() > 0 {
        highest_level_paths = highest_level_paths
            .into_iter()
            .map(|(n, funcs)| {
                Func::iter()
                    .filter_map(|func| match func.apply_reverse_if(n, reverse) {
                        Some(n) => {
                            let mut new_funcs = funcs.clone();
                            new_funcs.push(func);
                            Some((n, new_funcs))
                        }
                        None => None,
                    })
                    .collect::<Vec<(f64, Vec<Func>)>>()
            })
            .flatten()
            .collect();
        paths.extend(highest_level_paths.clone());
    }
    paths
}

pub struct GoalPaths {
    pub goal: f64,
    paths: HashMap<OrderedFloat<f64>, GoalPath>,
}
impl GoalPaths {
    fn new(goal: f64, nums: &[f64]) -> GoalPaths {
        let single_paths = expand_funcs(goal, true);

        let mut all_paths: Vec<(f64, GoalPath)> = single_paths
            .iter()
            .map(|(n, funcs)| (*n, GoalPath::new_single(funcs.clone())))
            .collect();
        for (i, og_num) in nums.iter().enumerate() {
            let used = Used::new_set(i);
            let num_expand = expand_funcs(*og_num, false);
            for ((num, num_funcs), (outer, outer_funcs), op) in
                iproduct!(num_expand.iter(), single_paths.iter(), Operation::iter())
            {
                let right_path = op.apply_reverse_left(*num, *outer);
                let left_path = op.apply_reverse_right(*num, *outer);
                if let Some(right_path) = right_path {
                    all_paths.push((
                        right_path.into(),
                        GoalPath::new_double(
                            outer_funcs.clone(),
                            num_funcs.clone(),
                            op.clone(),
                            OperationSide::Right,
                            used.clone(),
                            *og_num,
                        ),
                    ));
                }
                if let Some(left_path) = left_path {
                    all_paths.push((
                        left_path.into(),
                        GoalPath::new_double(
                            outer_funcs.clone(),
                            num_funcs.clone(),
                            op.clone(),
                            OperationSide::Left,
                            used.clone(),
                            *og_num,
                        ),
                    ));
                }
            }
        }
        let paths: HashMap<OrderedFloat<f64>, GoalPath> =
            all_paths
                .into_iter()
                .fold(HashMap::new(), |mut map, (n, path)| {
                    let n = n.into();
                    if let Some(old_path) = map.get(&n) {
                        if old_path.guess_score_delta() > path.guess_score_delta() {
                            map.insert(n, path);
                        }
                    } else {
                        map.insert(n, path);
                    }
                    map
                });

        GoalPaths { goal, paths }
    }
    fn get_path(&self, test: f64, used: &Used) -> Option<GoalPath> {
        if within_error(test, self.goal) {
            Some(GoalPath::new_empty())
        } else {
            let path = self.paths.get(&test.into()).cloned();
            if let Some(GoalPath::Double {
                used: path_used, ..
            }) = path
            {
                // make sure the used dont overlap
                if path_used.overlap(used) {
                    return None;
                }
            }
            path
        }
    }
    pub fn print_list(&self) {
        for (n, path) in self.paths.iter() {
            println!("{}: {}", n, path);
        }
    }
}

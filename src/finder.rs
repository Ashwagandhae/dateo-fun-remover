use itertools::iproduct;
use itertools::repeat_n;
use itertools::Itertools;
use math::within_error;
use rayon::prelude::*;
use std::collections::HashMap;
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

#[derive(Debug, Clone)]
struct Used(u8);

impl Used {
    fn new() -> Used {
        Used(0)
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
}
pub fn solution_with_least_funcs(
    goal_paths: &GoalPaths,
    atom: &Atom,
    atom_group: &AtomGroup,
    codon_index: usize,
    codon_count: usize,
    funcs: &Vec<Func>,
    distribution: &Vec<usize>,
) -> Option<(FuncAtom, u32)> {
    let val = atom_group.eval_with_funcs(codon_index, codon_count, funcs, distribution)?;
    let path = goal_paths.get_path(val)?;
    let min_funcs = funcs.len();
    let funcs = path.iter().chain(funcs.iter()).collect::<Vec<_>>();
    let distribution = repeat_n(0usize, path.len())
        .chain(distribution.iter().cloned())
        .collect::<Vec<_>>();
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
        let success = atom_group
            .eval_with_funcs(codon_index, codon_count, &new_funcs, &new_distribution)
            .map(|test| within_error(test, goal_paths.goal))
            .unwrap_or(false);
        if !success {
            continue;
        }
        if new_funcs.len() < min_funcs {
            return None;
        }
        // we know this will be the best solution, because we're starting from all zeros
        // in bit_mask, so we'll have the most funcs possible
        return Some((
            FuncAtom::new(atom.clone(), &new_funcs, &new_distribution),
            (new_funcs.len() - min_funcs) as u32,
        ));
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
        (atom, (codon_index, codon_count)): (&Atom, &(usize, usize)),
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

pub fn create_atoms(nums: &Vec<f64>) -> Vec<Atom> {
    fn rec(nums: &Vec<f64>, used: Used) -> Vec<Atom> {
        let mut ret_express = Vec::new();
        let nums_iter = nums.iter().enumerate().filter(|(i, _)| !used.get(*i));
        let other_nums_iter = nums_iter.clone();

        // num + num
        for (pair, op) in iproduct!(nums_iter.map(|(_, n)| n).combinations(2), Operation::iter()) {
            ret_express.push(Atom::new_express(pair[0], pair[1], op.clone()));
            if !op.is_commutative() && pair[0] != pair[1] {
                ret_express.push(Atom::new_express(pair[1], pair[0], op.clone()));
            }
        }

        if nums.len() - used.count() == 2 {
            return ret_express;
        }
        // num + expr
        for (i, num) in other_nums_iter {
            for (new_atom, op) in iproduct!(rec(nums, used.clone_set(i),), Operation::iter()) {
                ret_express.push(Atom::new_express(num, new_atom.clone(), op.clone()));
                if !op.is_commutative() && new_atom.eval().map(|n| n != *num).unwrap_or(true) {
                    ret_express.push(Atom::new_express(new_atom.clone(), num, op.clone()));
                }
            }
        }
        ret_express
    }
    rec(nums, Used::new())
        .into_iter()
        .filter(|atom| atom.eval_possible())
        .collect()
}

pub fn create_atom_store(nums: &Vec<f64>) -> AtomStore {
    AtomStore::new(create_atoms(nums))
}

pub fn create_goal_paths(goal: f64, depth: usize) -> GoalPaths {
    GoalPaths::new(goal, depth)
}

pub struct GoalPaths {
    pub goal: f64,
    paths: HashMap<OrderedFloat<f64>, Vec<Func>>,
}
impl GoalPaths {
    fn new(goal: f64, depth: usize) -> GoalPaths {
        let paths = (1..depth)
            .map(|current_depth| {
                itertools::repeat_n(Func::iter(), current_depth)
                    .multi_cartesian_product()
                    .map(|funcs| {
                        (
                            funcs.iter().rev().fold(Some(goal), |acc, func| {
                                acc.and_then(|n| func.apply_reversed(n))
                            }),
                            funcs,
                        )
                    })
                    .filter_map(|(n, funcs)| n.map(|n| (n, funcs)))
                    .collect::<Vec<(f64, Vec<Func>)>>()
            })
            .flatten()
            .fold(
                HashMap::<OrderedFloat<f64>, Vec<Func>>::new(),
                |mut map, (n, funcs)| {
                    // if there's already a path to this number, only keep the longer one
                    if let Some(existing) = map.get_mut(&n.into()) {
                        if existing.len() < funcs.len() {
                            *existing = funcs;
                        }
                    } else {
                        map.insert(n.into(), funcs);
                    }
                    map
                },
            );

        GoalPaths { goal, paths }
    }
    fn get_path(&self, test: f64) -> Option<Vec<Func>> {
        if within_error(test, self.goal) {
            Some(Vec::new())
        } else {
            self.paths.get(&test.into()).cloned()
        }
    }
    pub fn print_list(&self) {
        for (n, funcs) in self.paths.iter() {
            println!("{}: {:?}", n, funcs);
        }
    }
}

use itertools::iproduct;
use itertools::Itertools;
use math::matches_goal;
use rayon::prelude::*;
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
pub fn all_funcs_useful(goal: f64, atom: &Atom, funcs: &[Func], distribution: &[usize]) -> bool {
    if funcs.len() == 0 {
        return true;
    }
    // go through all possible arrays of 1s and 0s with length distribution.len() - 1
    // exclude the last one, since it's all 1s, which evals to true

    let ret = (0..(2u64.pow(distribution.len() as u32) - 1)).find(|bit_mask| {
        let mut new_funcs = Vec::new();
        let mut new_distribution = Vec::new();
        for (i, func_index) in distribution.iter().enumerate() {
            if bit_mask & (1 << i) != 0 {
                new_funcs.push(funcs[i].clone());
                new_distribution.push(*func_index);
            }
        }
        atom.eval_with_funcs(&new_funcs, &new_distribution)
            .map(|n| matches_goal(n, goal))
            .unwrap_or(false)
    });
    ret.is_none()
}

pub fn get_solution_in_group(
    func_count: &u32,
    goal: f64,
    atom_group: &AtomGroup,
) -> Option<FuncAtom> {
    atom_group
        .iter()
        .find_map(|(atom, (codon_index, codon_count))| {
            // atom_group.iter().find_map_any(|(atom, codon_index)| {
            Func::iter()
                .combinations_with_replacement(*func_count as usize)
                .find_map(|funcs| {
                    (0..atom.count_atoms() as usize)
                        .combinations_with_replacement(*func_count as usize)
                        .find(|distribution| {
                            atom_group
                                .eval_with_funcs(*codon_index, *codon_count, &funcs, distribution)
                                .map(|n| matches_goal(n, goal))
                                .unwrap_or(false)
                                && all_funcs_useful(goal, atom, &funcs, distribution)
                        })
                        .map(|distribution| FuncAtom::new(atom.clone(), &funcs, &distribution))
                })
        })
}
pub fn get_solution_with_score(score: u32, goal: f64, store: &AtomStore) -> Option<FuncAtom> {
    store
        .iter()
        .filter(|(base_score, _)| **base_score <= score)
        .find_map(|(base_score, atom_group)| {
            get_solution_in_group(&(score - base_score), goal, atom_group)
        })
}

fn create_atoms_rec(nums: &Vec<f64>, used: Used) -> Vec<Atom> {
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
        for (new_atom, op) in iproduct!(
            create_atoms_rec(nums, used.clone_set(i),),
            Operation::iter()
        ) {
            ret_express.push(Atom::new_express(num, new_atom.clone(), op.clone()));
            if !op.is_commutative() && new_atom.eval().map(|n| n != *num).unwrap_or(true) {
                ret_express.push(Atom::new_express(new_atom.clone(), num, op.clone()));
            }
        }
    }

    ret_express
}
pub fn create_atoms(nums: &Vec<f64>) -> Vec<Atom> {
    create_atoms_rec(nums, Used::new())
}

pub fn create_atom_store(nums: &Vec<f64>) -> AtomStore {
    AtomStore::new(create_atoms(nums))
}

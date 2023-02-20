use itertools::iproduct;
use itertools::Itertools;
use strum::IntoEnumIterator;

pub mod operation;
use operation::Operation;
pub mod function;
use function::Function;
pub mod atom;
use atom::Atom;
pub mod atom_store;
use atom_store::AtomStore;
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

pub fn get_solution(function_count: &u32, goal: f64, atom_group: &Vec<Atom>) -> Option<Atom> {
    for (atom, funcs) in iproduct!(
        atom_group.iter(),
        Function::iter().combinations_with_replacement(*function_count as usize)
    ) {
        for distribution in
            (0..atom.count_atoms() as usize).combinations_with_replacement(*function_count as usize)
        {
            let mut new_atom = atom.clone();
            new_atom.add_funcs(&funcs, &distribution);

            if new_atom.eval().map(|n| n == goal).unwrap_or(false) {
                return Some(new_atom);
            }
        }
    }
    None
}
pub fn get_solution_with_score(score: u32, goal: f64, store: &AtomStore) -> Option<Atom> {
    for (base_score, atom_group) in store.iter() {
        if *base_score > score {
            continue;
        }
        let needed_functions = score - base_score;
        let solution = get_solution(&needed_functions, goal, atom_group);
        if solution.is_some() {
            return solution;
        }
    }
    None
}

fn create_atoms_rec(nums: &Vec<f64>, used: Used) -> Vec<Atom> {
    let mut ret_express = Vec::new();
    let nums_iter = nums.iter().enumerate().filter(|(i, _)| !used.get(*i));
    let other_nums_iter = nums_iter.clone();

    for (pair, op) in iproduct!(nums_iter.map(|(_, n)| n).combinations(2), Operation::iter()) {
        // num + num
        ret_express.push(Atom::new_express(pair[0], pair[1], op.clone()));
        if !op.is_commutative() {
            ret_express.push(Atom::new_express(pair[1], pair[0], op.clone()));
        }
    }

    if nums.len() - used.count() == 2 {
        return ret_express;
    }
    for (i, num) in other_nums_iter {
        for (new_atom, op) in iproduct!(
            create_atoms_rec(nums, used.clone_set(i),),
            Operation::iter()
        ) {
            // num + num
            ret_express.push(Atom::new_express(num, new_atom.clone(), op.clone()));
            if !op.is_commutative() {
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
    let mut store = AtomStore::new();
    store.load_atoms(create_atoms(nums));
    store
}

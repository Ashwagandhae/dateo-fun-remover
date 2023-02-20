use itertools::iproduct;
use itertools::Itertools;
use strum::IntoEnumIterator;

mod operation;
use operation::Operation;
mod function;
use function::Function;
mod atom;
use atom::Atom;
mod atom_store;
use atom_store::AtomStore;
mod math;

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

pub struct Finder {
    nums: Vec<f64>,
    goal: f64,
    store: AtomStore,
}
impl Finder {
    pub fn new(nums: Vec<f64>, goal: f64) -> Finder {
        let mut finder = Finder {
            nums,
            goal,
            store: AtomStore::new(),
        };
        finder.load_atoms();

        finder
    }
    fn load_atoms(&mut self) {
        println!("loading atoms...");
        self.store.load_atoms(self.create_atoms(Used::new()));
        println!("atoms: {}", self.store.len());
    }

    pub fn get_solution(&self, function_count: &u32, atom_group: &Vec<Atom>) -> Option<Atom> {
        for (atom, funcs) in iproduct!(
            atom_group.iter(),
            Function::iter().combinations_with_replacement(*function_count as usize)
        ) {
            for distribution in (0..atom.count_atoms() as usize)
                .combinations_with_replacement(*function_count as usize)
            {
                let mut new_atom = atom.clone();
                new_atom.add_funcs(&funcs, &distribution);

                if new_atom.eval().map(|n| n == self.goal).unwrap_or(false) {
                    return Some(new_atom);
                }
            }
        }
        None
    }
    pub fn get_solution_with_score(&self, score: u32) -> Option<Atom> {
        for (base_score, atom_group) in self.store.iter() {
            if *base_score > score {
                continue;
            }
            let needed_functions = score - base_score;
            let solution = self.get_solution(&needed_functions, atom_group);
            if solution.is_some() {
                return solution;
            }
        }
        None
    }
    pub fn find_solutions(&self, max_score: u32) {
        for score in 0..max_score {
            println!("searching for solutions with score {}", score);
            let solution = self.get_solution_with_score(score);
            if let Some(solution) = solution {
                println!("found solution with score {}: {}", score, solution);
                solution.eval_verbose();
            }
        }
    }
    fn create_atoms(&self, used: Used) -> Vec<Atom> {
        let mut ret_express = Vec::new();
        let nums = self.nums.iter().enumerate().filter(|(i, _)| !used.get(*i));
        let other_nums = nums.clone();

        for (pair, op) in iproduct!(nums.map(|(_, n)| n).combinations(2), Operation::iter()) {
            // num + num
            ret_express.push(Atom::new_express(pair[0], pair[1], op.clone()));
            if !op.is_commutative() {
                ret_express.push(Atom::new_express(pair[1], pair[0], op.clone()));
            }
        }

        if self.nums.len() - used.count() == 2 {
            return ret_express;
        }
        for (i, num) in other_nums {
            for (new_atom, op) in iproduct!(self.create_atoms(used.clone_set(i)), Operation::iter())
            {
                // num + num
                ret_express.push(Atom::new_express(num, new_atom.clone(), op.clone()));
                if !op.is_commutative() {
                    ret_express.push(Atom::new_express(new_atom.clone(), num, op.clone()));
                }
            }
        }

        ret_express
    }
}

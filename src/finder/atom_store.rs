use crate::finder::atom::Atom;
use crate::finder::func::Func;
use crate::finder::operation::Operation;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct AtomStore {
    atom_groups: HashMap<u32, AtomGroup>,
    sorted_scores: Vec<u32>,
}

impl AtomStore {
    pub fn new(atoms: Vec<Atom>) -> AtomStore {
        let mut atom_map = HashMap::new();
        for atom in atoms {
            atom_map
                .entry(atom.score())
                .or_insert(Vec::new())
                .push(atom);
        }
        let mut sorted_scores: Vec<u32> = atom_map.keys().copied().collect();
        sorted_scores.sort();
        let atom_groups = atom_map
            .iter()
            .map(|(score, atoms)| (*score, AtomGroup::new(atoms)))
            .collect();
        AtomStore {
            atom_groups,
            sorted_scores,
        }
    }
    pub fn len(&self) -> usize {
        self.atom_groups.iter().map(|(_, v)| v.len()).sum()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &AtomGroup)> {
        // release from largest base_score to smallest
        // minimizing the number of functions needed
        self.sorted_scores.iter().rev().map(move |base_score| {
            let atoms = self.atom_groups.get(base_score).unwrap();
            (base_score, atoms)
        })
    }
}

#[derive(Debug, Clone)]
pub enum Codon {
    Express {
        left: usize,
        right: usize,
        op: Operation,
        calc_box_save: usize,
        func_index: usize,
    },
    Number {
        num: f64,
        calc_box_save: usize,
        func_index: usize,
    },
}

fn codons_from_atom_rec(
    atom: &Atom,
    codons: &mut Vec<Codon>,
    calc_box_index: &mut usize,
    i: &mut usize,
) -> usize {
    let func_index = i.clone();
    *i += 1;
    match atom {
        Atom::Express { left, right, op } => {
            let left_index = codons_from_atom_rec(&*left, codons, calc_box_index, i);
            let right_index = codons_from_atom_rec(&*right, codons, calc_box_index, i);
            codons.push(Codon::Express {
                left: left_index,
                right: right_index,
                op: op.clone(),
                func_index,
                // always store it in the left calc box (this choice is arbitrary)
                calc_box_save: left_index,
            });
            left_index
        }
        Atom::Number(n) => {
            codons.push(Codon::Number {
                num: *n,
                calc_box_save: *calc_box_index,
                func_index,
            });
            let calc_box_index_saved = calc_box_index.clone();
            *calc_box_index += 1;
            calc_box_index_saved
        }
    }
}
fn codons_from_atom(atom: &Atom) -> Vec<Codon> {
    let mut codons = Vec::new();
    let mut calc_box_index = 0;
    codons_from_atom_rec(atom, &mut codons, &mut calc_box_index, &mut 0);
    // make last codon always put the result in the first calc box
    // this is so that the last codon can be used to evaluate the whole expression
    if let Some(Codon::Express { calc_box_save, .. }) = codons.last_mut() {
        *calc_box_save = 0;
    }
    codons
}

pub struct AtomGroup {
    atoms: Vec<Atom>,
    pub codon_info: Vec<(usize, usize)>,
    codons: Vec<Codon>,
}
impl AtomGroup {
    fn new(atoms: &[Atom]) -> AtomGroup {
        let mut codons = Vec::new();
        let mut codon_info = Vec::new();
        for atom in atoms {
            let og_len = codons.len();
            let new_codons = codons_from_atom(atom);
            codon_info.push((og_len, new_codons.len()));
            codons.extend(new_codons);
        }
        AtomGroup {
            atoms: atoms.to_vec(),
            codon_info,
            codons,
        }
    }
    fn len(&self) -> usize {
        self.atoms.len()
    }
    pub fn par_iter(&self) -> impl ParallelIterator<Item = (&Atom, &(usize, usize))> {
        self.atoms.par_iter().zip(self.codon_info.par_iter())
    }
    pub fn iter(&self) -> impl Iterator<Item = (&Atom, &(usize, usize))> {
        self.atoms.iter().zip(self.codon_info.iter())
    }
    // fn bulk_eval_with_funcs(
    //     &self,
    //     atom_index: usize,
    //     func_count: u32,
    //     matches_goal: &dyn Fn(Option<f64>) -> bool,
    // ) {
    //     let atom = &self.atoms[atom_index];
    //     let atoms_count = atom.count_atoms();
    //     Func::iter()
    //         .combinations_with_replacement(func_count as usize)
    //         .find_map(|funcs| {
    //             (0..atoms_count as usize)
    //                 .combinations_with_replacement(func_count as usize)
    //                 .map(|distribution| {

    //                 })
    //                 .find(matches_goal)
    //         });
    // }
    #[allow(invalid_value)]
    pub fn eval_with_funcs(
        &self,
        codon_index: usize,
        codon_count: usize,
        funcs: &[Func],
        distribution: &[usize],
    ) -> Option<f64> {
        let mut calc_box: [f64; 5];
        // leave calc_box uninitialized
        // this is safe because we will always write to it before reading from it
        // the compiler can't know that we will set the values before reading them
        unsafe { calc_box = std::mem::MaybeUninit::uninit().assume_init() };

        for codon in self.codons[codon_index..codon_index + codon_count].iter() {
            let (num, calc_box_save, func_index) = match codon {
                Codon::Express {
                    left,
                    right,
                    op,
                    calc_box_save,
                    func_index,
                } => {
                    let left = calc_box[*left];
                    let right = calc_box[*right];
                    (op.apply(left, right)?, *calc_box_save, *func_index)
                }
                Codon::Number {
                    num,
                    calc_box_save,
                    func_index,
                } => (*num, *calc_box_save, *func_index),
            };
            let distributed = Atom::distribute_funcs(funcs, distribution, func_index);
            let num = distributed.fold(Some(num), |acc, func| func.apply(acc?));
            calc_box[calc_box_save] = num?;
        }
        Some(calc_box[0])
    }
}

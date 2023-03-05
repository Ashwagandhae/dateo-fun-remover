use crate::finder::atom::Atom;
use crate::finder::codon::{codons_from_atom, Codon, CodonVal};
use crate::finder::func::Func;
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
        // release from smallest base_score to largest
        self.sorted_scores.iter().rev().map(move |base_score| {
            let atoms = self.atom_groups.get(base_score).unwrap();
            (base_score, atoms)
        })
    }
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
    #[allow(dead_code)]
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
            let Codon {
                val: codon,
                calc_box_save,
                func_index,
            } = codon;
            let (num, calc_box_save, func_index) = match codon {
                CodonVal::Express { left, right, op } => {
                    let left = calc_box[*left];
                    let right = calc_box[*right];
                    (op.apply(left, right)?, *calc_box_save, *func_index)
                }
                CodonVal::Number { num } => (*num, *calc_box_save, *func_index),
            };
            let distributed = Atom::distribute_funcs(funcs, distribution, func_index);
            let num = distributed.fold(Some(num), |acc, func| func.apply(acc?));
            calc_box[calc_box_save] = num?;
        }
        Some(calc_box[0])
    }
}

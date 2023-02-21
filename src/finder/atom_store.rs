use crate::finder::atom::Atom;
use std::collections::HashMap;

pub struct AtomStore {
    atoms: HashMap<u32, Vec<Atom>>,
    sorted_scores: Vec<u32>,
}

impl AtomStore {
    pub fn new() -> AtomStore {
        AtomStore {
            atoms: HashMap::new(),
            sorted_scores: Vec::new(),
        }
    }
    pub fn load_atoms(&mut self, atoms: Vec<Atom>) {
        for atom in atoms {
            self.atoms
                .entry(atom.score())
                .or_insert(Vec::new())
                .push(atom);
        }
        self.sorted_scores = self.atoms.keys().copied().collect();
        self.sorted_scores.sort();
    }
    pub fn len(&self) -> usize {
        self.atoms.iter().map(|(_, v)| v.len()).sum()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &Vec<Atom>)> {
        // release from largest base_score to smallest
        // minimizing the number of functions needed
        self.sorted_scores.iter().rev().map(move |base_score| {
            let atoms = self.atoms.get(base_score).unwrap();
            (base_score, atoms)
        })
    }
}

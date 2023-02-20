use crate::finder::atom::Atom;
use std::collections::HashMap;

pub struct AtomStore {
    atoms: HashMap<u32, Vec<Atom>>,
}

impl AtomStore {
    pub fn new() -> AtomStore {
        AtomStore {
            atoms: HashMap::new(),
        }
    }
    pub fn load_atoms(&mut self, atoms: Vec<Atom>) {
        for atom in atoms {
            self.atoms
                .entry(atom.score())
                .or_insert(Vec::new())
                .push(atom);
        }
    }
    pub fn len(&self) -> usize {
        self.atoms.iter().map(|(_, v)| v.len()).sum()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &Vec<Atom>)> {
        self.atoms.iter()
    }
}

use crate::finder::atom::{Atom, AtomVal};
use crate::finder::operation::Operation;
use std::cmp::max;

#[derive(Debug, Clone)]
pub enum CodonVal {
    Express {
        left: usize,
        right: usize,
        op: Operation,
    },
    Number {
        num: f64,
    },
}

#[derive(Debug, Clone)]
pub struct Codon {
    pub val: CodonVal,
    pub calc_box_save: usize,
    pub func_index: usize,
}

pub fn codons_from_atom(atom: &Atom) -> Vec<Codon> {
    #[derive(Debug, Clone)]
    enum DependCodonVal {
        Express {
            left_id: usize,
            right_id: usize,
            op: Operation,
        },
        Number {
            num: f64,
            calc_box_save: usize,
        },
    }
    #[derive(Debug, Clone)]
    struct DependCodon {
        val: DependCodonVal,
        func_index: usize,
        id: usize,
    }

    fn rec(
        atom: &Atom,
        codons: &mut Vec<DependCodon>,
        calc_box_index: &mut usize,
        i: &mut usize,
    ) -> usize {
        let func_index = i.clone();
        if !atom.immune {
            *i += 1;
        }
        match &atom.val {
            AtomVal::Express { left, right, op } => {
                let left_id = rec(&*left, codons, calc_box_index, i);
                let right_id = rec(&*right, codons, calc_box_index, i);
                codons.push(DependCodon {
                    val: DependCodonVal::Express {
                        left_id,
                        right_id,
                        op: op.clone(),
                    },
                    func_index,
                    id: codons.len(),
                })
            }
            AtomVal::Number(n) => {
                codons.push(DependCodon {
                    val: DependCodonVal::Number {
                        num: *n,
                        calc_box_save: *calc_box_index,
                    },
                    func_index,
                    id: codons.len(),
                });
                *calc_box_index += 1;
            }
        }
        codons.len() - 1
    }

    fn index_from_id(depend_codons: &Vec<DependCodon>, id: usize) -> usize {
        depend_codons
            .iter()
            .enumerate()
            .find(|(_, c)| c.id == id)
            .unwrap()
            .0
    }

    // why do this? because we want to evaluate power/roots first, because they
    // are the most likely to fail, and we want to fail fast
    fn move_codon_front(depend_codons: &mut Vec<DependCodon>, index: usize) {
        let new_index = match depend_codons[index].val {
            DependCodonVal::Express {
                left_id, right_id, ..
            } => {
                let left = index_from_id(depend_codons, left_id);
                let right = index_from_id(depend_codons, right_id);
                move_codon_front(depend_codons, left);
                move_codon_front(depend_codons, right);
                let left = index_from_id(depend_codons, left_id);
                let right = index_from_id(depend_codons, right_id);
                // then move this codon front
                // it can't go further front than the left and right
                max(left, right) + 1
            }

            DependCodonVal::Number { .. } => {
                // numbers have no dependencies, so just move them to the front
                0
            }
        };
        if new_index < index {
            let codon = depend_codons.remove(index);
            depend_codons.insert(new_index, codon);
        }
    }

    fn reorder_depend_codons(depend_codons: &mut Vec<DependCodon>) {
        let priority_codon_ids: Vec<usize> = depend_codons
            .iter()
            .filter(|codon| {
                matches!(
                    codon.val,
                    DependCodonVal::Express {
                        op: Operation::Power | Operation::Root,
                        ..
                    }
                )
            })
            .map(|codon| codon.id)
            // loop thru depend_codons backwards so that atoms farther down the tree
            // are moved to the front of the list last, giving them priority
            .rev()
            .collect();
        for id in priority_codon_ids.iter() {
            let index = index_from_id(depend_codons, *id);
            move_codon_front(depend_codons, index);
        }
    }

    let mut depend_codons = Vec::new();
    let mut calc_box_index = 0;
    rec(atom, &mut depend_codons, &mut calc_box_index, &mut 0);

    reorder_depend_codons(&mut depend_codons);

    // convert depend_codons to codons
    let mut codons: Vec<Codon> = Vec::new();
    for depend_codon in depend_codons.iter() {
        let codon = match &depend_codon.val {
            DependCodonVal::Express {
                left_id,
                right_id,
                op,
            } => {
                // get the calc_box_save of the left codon and right codon
                let left_calc_box = codons[index_from_id(&depend_codons, *left_id)].calc_box_save;
                let right_calc_box = codons[index_from_id(&depend_codons, *right_id)].calc_box_save;
                Codon {
                    val: CodonVal::Express {
                        left: left_calc_box,
                        right: right_calc_box,
                        op: op.clone(),
                    },
                    calc_box_save: left_calc_box,
                    func_index: depend_codon.func_index,
                }
            }
            DependCodonVal::Number { num, calc_box_save } => Codon {
                val: CodonVal::Number { num: *num },
                calc_box_save: *calc_box_save,
                func_index: depend_codon.func_index,
            },
        };
        codons.push(codon);
    }

    // make last codon always put the result in the first calc box
    // so that the last codon can be used to evaluate the whole expression
    codons.last_mut().unwrap().calc_box_save = 0;
    codons
}

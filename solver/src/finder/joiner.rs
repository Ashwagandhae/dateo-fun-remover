use ahash::AHashMap as HashMap;
use ordered_float::OrderedFloat;
// use rustc_hash::FxHashMap as HashMap;

use itertools::Itertools;

use super::atom::Atom;
use super::score::Score;
use super::tree::{expand_funcs, Arena, Kind, Link, Path, Val};

use super::tree_shapes::*;

pub struct Joiner {
    up: Arena,
    down: Arena,
}
pub enum AtomFilter {
    None,
    MinScore(u8),
}
impl Joiner {
    fn from_strings(up: &str, down: &str) -> Self {
        let up = Arena::from_string(up);
        let down = Arena::from_string(down);
        Self { up, down }
    }
    #[inline(never)]
    pub fn solve<'a>(
        &'a mut self,
        nums: &[f64],
        goal: f64,
        depth: usize,
        mut atom_filter: AtomFilter,
        memo: &'a mut Memo,
    ) -> impl Iterator<Item = (Score, Atom)> + 'a {
        let up_perm_map = self.up.perm_map();
        let down_perm_map = self.down.perm_map();
        let perm_middle = up_perm_map.len();
        let perm_map = [&up_perm_map[..], &down_perm_map[..]].concat();

        set_nums_and_goal_in_memo(nums, goal, depth, memo);

        get_perms(nums, &perm_map)
            .into_iter()
            .flat_map(move |perm| {
                self.up.populate(&perm[..perm_middle], None, memo);
                self.up.solve(depth, memo);

                self.down.populate(&perm[perm_middle..], Some(goal), memo);
                self.down.solve(depth, memo);

                find_val_intersects(&self.up.keys[0], &self.down.keys[0], &memo)
                    .filter_map(|(up_val, down_val)| {
                        let score = (up_val.score + down_val.score).resolve();
                        if let AtomFilter::MinScore(min_score) = atom_filter {
                            if score.score() <= min_score {
                                return None;
                            }
                        }
                        let atom = join_vals(&up_val, &self.up, &down_val, &self.down, &memo);
                        if !atom.test(goal) {
                            return None;
                        }
                        if let AtomFilter::MinScore(min_score) = &mut atom_filter {
                            *min_score = score.score();
                        }

                        Some((score, atom))
                    })
                    .collect_vec()
            })
    }
}

fn join_vals(up_val: &Val, up: &Arena, down_val: &Val, down: &Arena, memo: &Memo) -> Atom {
    let sub_atom = val_to_atom(up_val, 0, up, memo);
    // println!("sub_atom: {}\n", sub_atom);
    let mut atom = val_to_atom_rev(down_val, 0, down, memo);
    atom.fill_hole(sub_atom);
    // println!("atom: {}\n", atom);
    atom
}
fn val_to_atom(val: &Val, id: usize, arena: &Arena, memo: &Memo) -> Atom {
    let node = arena.get(id);
    // println!(
    //     "val_to_atom: ({} {} -> {}) {:?} {:?} {:?}",
    //     id, val.origin, val.num, node.link, node.kind, val.path
    // );
    let mut atom = match &val.path {
        Path::Leaf => Atom::new(val.origin),
        Path::Combine { op, left, right } => {
            let Link::Branch(left_id, right_id) = node.link else { unreachable!() };
            let left = val_to_atom(
                &arena.get_vals_from_memo(left_id, memo)[*left],
                left_id,
                arena,
                memo,
            );
            let right = val_to_atom(
                &arena.get_vals_from_memo(right_id, memo)[*right],
                right_id,
                arena,
                memo,
            );
            Atom::new_express(left, right, op.clone())
        }
    };
    atom.funcs = val.funcs.clone();
    atom
}
fn val_to_atom_rev(val: &Val, id: usize, arena: &Arena, memo: &Memo) -> Atom {
    let mut id_val_map: Vec<Option<Val>> = vec![None; arena.len()];
    fn fill_map_rec(
        val: &Val,
        id: usize,
        arena: &Arena,
        id_val_map: &mut Vec<Option<Val>>,
        memo: &Memo,
    ) {
        let node = arena.get(id);
        id_val_map[id] = Some(val.clone());
        match &val.path {
            Path::Leaf => {}
            Path::Combine { left, right, .. } => {
                let Link::Branch(left_id, right_id) = node.link else { unreachable!() };
                fill_map_rec(
                    &arena.get_vals_from_memo(left_id, memo)[*left],
                    left_id,
                    arena,
                    id_val_map,
                    memo,
                );
                fill_map_rec(
                    &arena.get_vals_from_memo(right_id, memo)[*right],
                    right_id,
                    arena,
                    id_val_map,
                    memo,
                );
            }
        }
    }
    fill_map_rec(val, id, arena, &mut id_val_map, memo);
    let id_val_map = id_val_map.into_iter().flatten().collect::<Vec<_>>();
    fn rec(val: &Val, id: usize, arena: &Arena, id_val_map: &[Val], memo: &Memo) -> Atom {
        let node = arena.get(id);
        // println!(
        //     "rev: ({} {} -> {}) {:?} {:?} {:?}",
        //     id, val.origin, val.num, node.link, node.kind, val.path
        // );
        if matches!(node.kind, Kind::Num) {
            return val_to_atom(val, id, arena, memo);
        };

        let mut atom = match node.parent {
            Some(parent_id) => {
                let parent = arena.get(parent_id);
                let Link::Branch(left_id, _) = parent.link else { unreachable!() };

                let parent_val = id_val_map[parent_id].clone();
                let sibling_val = id_val_map[left_id].clone();

                let Path::Combine { op, .. } = parent_val.path.clone() else { unreachable!() };

                let parent_atom = rec(&parent_val, parent_id, arena, id_val_map, memo);
                let sibling_atom = rec(&sibling_val, left_id, arena, id_val_map, memo);
                Atom::new_express(sibling_atom, parent_atom, op)
            }
            None => {
                // we've reached the root
                Atom::new_hole()
            }
        };
        for func in val.funcs.reverse().iter() {
            atom.funcs.push(func);
        }
        atom
    }
    let goal_id = arena.get_goal_id();
    rec(&id_val_map[goal_id], goal_id, arena, &id_val_map, memo)
}

fn get_perms(nums: &[f64], perm_map: &[bool]) -> Vec<Vec<f64>> {
    nums.iter()
        .cloned()
        .permutations(nums.len())
        .filter(|perm| {
            perm.iter()
                .tuple_windows()
                .zip(perm_map.iter())
                .all(|((curr, next), keep)| {
                    // we need to remove 1/2 of the pairs of numbers
                    *keep || curr < next
                })
        })
        .collect()
}

pub fn get_joiners(num_count: usize) -> Vec<Joiner> {
    match num_count {
        1 => TREE_1.iter(),
        2 => TREE_2.iter(),
        3 => TREE_3.iter(),
        4 => TREE_4.iter(),
        5 => TREE_5.iter(),
        _ => panic!("unsupported tree"),
    }
    .map(|(up, down)| Joiner::from_strings(up, down))
    .collect()
}

fn set_nums_and_goal_in_memo(nums: &[f64], goal: f64, depth: usize, memo: &mut Memo) {
    for num in nums {
        let origin_val = Val::new_pure_leaf(*num, true);
        let num_vals = expand_funcs(*num, false, depth)
            .into_iter()
            .map(|(num, funcs)| origin_val.clone_with_funcs(num, funcs))
            .chain(std::iter::once(origin_val.clone()))
            .collect::<Vec<_>>();
        memo.insert(format!("N {}", num), num_vals);
    }
    let origin_val = Val::new_pure_leaf(goal, false);
    let goal_vals = expand_funcs(goal, true, depth)
        .into_iter()
        .map(|(num, funcs)| origin_val.clone_with_funcs(num, funcs))
        .chain(std::iter::once(origin_val.clone()))
        .collect::<Vec<_>>();
    memo.insert(format!("G {}", goal), goal_vals);
}

#[inline(never)]
fn find_val_intersects<'a>(
    key_1: &str,
    key_2: &str,
    memo: &'a Memo,
) -> impl Iterator<Item = (Val, Val)> + 'a {
    let vals_len = memo.get(key_1).unwrap().len();
    let other_vals_len = memo.get(key_2).unwrap().len();

    let switch = vals_len > other_vals_len;

    let (longer_key, shorter_key) = if switch {
        (key_1, key_2)
    } else {
        (key_2, key_1)
    };

    let longer_vals = memo.get(longer_key).unwrap();
    let shorter_vals = memo.get(shorter_key).unwrap();

    let longer_val_map = memo.get_map(longer_key);

    shorter_vals.iter().filter_map(move |shorter_val| {
        longer_val_map
            .get(&OrderedFloat(shorter_val.num))
            .map(|val_i| {
                let longer_val = &longer_vals[*val_i];
                if switch {
                    (longer_val.clone(), shorter_val.clone())
                } else {
                    (shorter_val.clone(), longer_val.clone())
                }
            })
    })
}

use std::cell::RefCell;
use std::rc::Rc;
type ValMap = HashMap<OrderedFloat<f64>, usize>;
pub struct Memo {
    map: HashMap<String, Vec<Val>>,
    map_map: RefCell<HashMap<String, Rc<ValMap>>>,
}

impl Memo {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
            map_map: RefCell::new(HashMap::default()),
        }
    }

    pub fn get(&self, key: &str) -> Option<&[Val]> {
        self.map.get(key).map(|vals| vals.as_slice())
    }

    pub fn get_map<'a>(&self, key: &str) -> Rc<ValMap> {
        if !self.map_map.borrow().contains_key(key) {
            let vals = self.map.get(key).expect("key not found");
            let val_map = vals
                .iter()
                .enumerate()
                .map(|(i, val)| (val.num.into(), i))
                .collect::<ValMap>();
            self.map_map
                .borrow_mut()
                .insert(key.to_string(), Rc::new(val_map));
        }
        self.map_map.borrow().get(key).unwrap().clone()
    }

    pub fn insert(&mut self, key: String, val: Vec<Val>) {
        self.map.insert(key, val);
    }
}

use hashbrown::HashMap;

use itertools::Itertools;

use super::atom::Atom;
use super::tree::{expand_funcs, Arena, Kind, Link, Path, Val};

pub struct Joiner {
    up: Arena,
    down: Arena,
}
impl Joiner {
    fn from_strings(up: &str, down: &str) -> Self {
        Self {
            up: Arena::from_string(up),
            down: Arena::from_string(down),
        }
    }
    #[inline(never)]
    pub fn solve(&mut self, nums: &[f64], goal: f64, depth: usize, max_score: &mut u32) {
        let up_perm_map = self.up.perm_map();
        let down_perm_map = self.down.perm_map();
        let perm_middle = up_perm_map.len();
        let perm_map = [&up_perm_map[..], &down_perm_map[..]].concat();
        let mut memo = HashMap::new();

        set_nums_and_goal_in_memo(nums, goal, depth, &mut memo);

        for perm in get_perms(nums, &perm_map) {
            self.up.populate(&perm[..perm_middle], None, &mut memo);
            self.up.solve(depth, &mut memo);

            self.down
                .populate(&perm[perm_middle..], Some(goal), &mut memo);
            self.down.solve(depth, &mut memo);

            let vals = &mut self.up.get_vals_from_memo_mut(0, &mut memo);
            sort_vals(vals);
            let other_vals = &mut self.down.get_vals_from_memo_mut(0, &mut memo);
            sort_vals(other_vals);

            let vals = self.up.get_vals_from_memo(0, &memo);
            let other_vals = self.down.get_vals_from_memo(0, &memo);

            let intersects = find_val_intersects(vals, other_vals);
            for (up_val, down_val) in intersects {
                let atom =
                    join_vals(&up_val, &self.up, &down_val, &self.down, &memo).simplify(goal);
                if let Some(atom) = atom {
                    if atom.score() > *max_score {
                        println!("atom with score {}: {}", atom.score(), atom);
                        atom.eval_verbose();
                        *max_score = atom.score();
                    }
                }
            }
        }
    }
}

fn join_vals(
    up_val: &Val,
    up: &Arena,
    down_val: &Val,
    down: &Arena,
    memo: &HashMap<String, Vec<Val>>,
) -> Atom {
    let sub_atom = val_to_atom(up_val, 0, up, memo);
    // println!("sub_atom: {}\n", sub_atom);
    let atom = val_to_atom_rev(down_val, 0, down, sub_atom, memo);
    // println!("atom: {}\n", atom);
    atom
}
fn val_to_atom(val: &Val, id: usize, arena: &Arena, memo: &HashMap<String, Vec<Val>>) -> Atom {
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
fn val_to_atom_rev(
    val: &Val,
    id: usize,
    arena: &Arena,
    sub_atom: Atom,
    memo: &HashMap<String, Vec<Val>>,
) -> Atom {
    let mut id_val_map: Vec<Option<Val>> = vec![None; arena.len()];
    fn fill_map_rec(
        val: &Val,
        id: usize,
        arena: &Arena,
        id_val_map: &mut Vec<Option<Val>>,
        memo: &HashMap<String, Vec<Val>>,
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
    fn rec(
        val: &Val,
        id: usize,
        arena: &Arena,
        id_val_map: &[Val],
        sub_atom: &Atom,
        memo: &HashMap<String, Vec<Val>>,
    ) -> Atom {
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

                let parent_atom = rec(&parent_val, parent_id, arena, id_val_map, sub_atom, memo);
                let sibling_atom = rec(&sibling_val, left_id, arena, id_val_map, sub_atom, memo);
                Atom::new_express(sibling_atom, parent_atom, op)
            }
            None => {
                // we've reached the root
                sub_atom.clone()
            }
        };
        for func in val.funcs.reverse().iter() {
            atom.funcs.push(func);
        }
        atom
    }
    let goal_id = arena.get_goal_id();
    rec(
        &id_val_map[goal_id],
        goal_id,
        arena,
        &id_val_map,
        &sub_atom,
        memo,
    )
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

pub fn create_base_trees() -> Vec<Joiner> {
    vec![
        (
            r"
   O
  / \
  N O
   / \
   N O
",
            r"
  H
 / \
 N H
  / \
  N G
",
        ),
        (
            r"
  O
 / \
 N O
  / \
  N O
",
            r"
   H
  / \
  O G
 / \
 N N
",
        ),
        (
            r"
     O
   /   \
   O   O
  / \ / \
  N N N N
",
            r"
   H
  / \
  N G

",
        ),
    ]
    .into_iter()
    .map(|(up, down)| Joiner::from_strings(up, down))
    .collect()
}

fn sort_vals(vals: &mut [Val]) {
    vals.sort_unstable_by(|a, b| a.num.partial_cmp(&b.num).unwrap());
}

#[inline(never)]
fn find_val_intersects<'a>(
    vals: &'a [Val],
    other_vals: &'a [Val],
) -> impl Iterator<Item = (Val, Val)> + 'a {
    // println!("intersecting {} and {} vals", vals.len(), other_vals.len());
    let mut i = 0;
    let mut other_i = 0;
    std::iter::from_fn(move || loop {
        if i >= vals.len() || other_i >= other_vals.len() {
            return None;
        }
        let val = &vals[i];
        let other_val = &other_vals[other_i];

        if val.num == other_val.num {
            i += 1;
            other_i += 1;
            break Some((val.clone(), other_val.clone()));
        } else if val.num < other_val.num {
            i += 1;
            continue;
        } else {
            other_i += 1;
            continue;
        }
    })
}

fn set_nums_and_goal_in_memo(
    nums: &[f64],
    goal: f64,
    depth: usize,
    memo: &mut HashMap<String, Vec<Val>>,
) {
    for num in nums {
        let origin_val = Val::new_pure_leaf(*num);
        let num_vals = expand_funcs(*num, false, depth)
            .into_iter()
            .map(|(num, funcs)| origin_val.clone_with_funcs(num, funcs))
            .chain(std::iter::once(origin_val.clone()))
            .collect::<Vec<_>>();
        memo.insert(format!("N {}", num), num_vals);
    }
    let origin_val = Val::new_pure_leaf(goal);
    let goal_vals = expand_funcs(goal, true, depth)
        .into_iter()
        .map(|(num, funcs)| origin_val.clone_with_funcs(num, funcs))
        .chain(std::iter::once(origin_val.clone()))
        .collect::<Vec<_>>();
    memo.insert(format!("G {}", goal), goal_vals);
}

// #[inline(never)]
// fn find_val_intersects(vals: &[Val], other_vals: &[Val]) -> Vec<(Val, Val)> {
//     let (longer_vals, shorter_vals) = if vals.len() < other_vals.len() {
//         (vals, other_vals)
//     } else {
//         (other_vals, vals)
//     };
//     type ValMap = HashMap<OrderedFloat<f64>, usize>;
//     let mut res = Vec::with_capacity(shorter_vals.len().saturating_sub(1));
//     let vals_map: ValMap = shorter_vals
//         .iter()
//         .enumerate()
//         .map(|(i, val)| (val.num.into(), i))
//         .collect();
//     for val in longer_vals {
//         if let Some(val_i) = vals_map.get(&OrderedFloat(val.num)) {
//             // keep the order
//             if vals.len() < other_vals.len() {
//                 res.push((val.clone(), shorter_vals[*val_i].clone()));
//             } else {
//                 res.push((shorter_vals[*val_i].clone(), val.clone()));
//             }
//         }
//     }
//     res
// }

// use bloom::BloomFilter;
// use bloom::ASMS;
// #[inline(never)]
// fn find_val_intersects(vals: &[Val], other_vals: &[Val]) -> Vec<(Val, Val)> {
//     type ValMap<'a> = HashMap<OrderedFloat<f64>, &'a Val>;
//     let mut res = Vec::new();

//     let mut filter = BloomFilter::with_rate(0.01, vals.len() as u32);
//     let mut val_map: ValMap = HashMap::new();
//     for val in vals {
//         filter.insert(&OrderedFloat(val.num));
//         val_map.insert(OrderedFloat(val.num), val);
//     }
//     for other_val in other_vals {
//         let key = &OrderedFloat(other_val.num);
//         if filter.contains(key) {
//             if let Some(val) = val_map.get(key) {
//                 res.push(((*val).clone(), other_val.clone()));
//             }
//         }
//     }
//     res
// }

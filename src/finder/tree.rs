use itertools::{iproduct, Itertools};
use strum::IntoEnumIterator;

use super::func::Func;
use super::func_list::FuncList;
use super::joiner::Memo;
use super::operation::Operation;

#[derive(Debug, Clone)]
pub enum Path {
    Leaf,
    Combine {
        op: Operation,
        left: usize,
        right: usize,
    },
}

#[derive(Debug, Clone)]
pub struct Val {
    pub num: f64,
    pub origin: f64,
    pub score: u32,
    pub funcs: FuncList,
    pub path: Path,
}

impl Val {
    pub fn new_pure_leaf(num: f64) -> Self {
        Self {
            num,
            origin: num,
            score: 0,
            funcs: FuncList::new(),
            path: Path::Leaf,
        }
    }
    pub fn clone_with_funcs(&self, num: f64, funcs: FuncList) -> Self {
        Self {
            num,
            origin: self.num,
            score: self.score + funcs.len() as u32,
            funcs,
            path: self.path.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Link {
    Branch(usize, usize),
    Leaf,
}
#[derive(Debug, Clone)]
pub enum Kind {
    Goal,
    Num,
}
#[derive(Debug, Clone)]
pub struct Node {
    pub link: Link,
    pub parent: Option<usize>,
    pub kind: Kind,
}
impl Node {
    fn new(kind: Kind, parent: Option<usize>) -> Self {
        Self {
            link: Link::Leaf,
            parent: parent,
            kind,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Arena {
    nodes: Vec<Node>,
    pub keys: Vec<String>,
}

impl Arena {
    fn new() -> Self {
        Self {
            nodes: vec![],
            keys: vec![],
        }
    }
    fn add_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    fn add_new(&mut self, kind: Kind, parent: Option<usize>) -> usize {
        self.add_node(Node::new(kind, parent))
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Node {
        &mut self.nodes[index]
    }
    pub fn get(&self, index: usize) -> &Node {
        &self.nodes[index]
    }
    pub fn from_string(s: &str) -> Arena {
        let mut arena = Arena::new();
        let mut lines = s.lines().filter(|line| !line.trim().is_empty()).step_by(2);
        let first_line = lines.next().unwrap();
        let mut nodes = vec![arena.add_new(
            if first_line.contains("H") || first_line.contains("G") {
                Kind::Goal
            } else {
                Kind::Num
            },
            None,
        )];

        for line in lines {
            let mut new_nodes = vec![];
            for (i, (left, right)) in line.split_ascii_whitespace().tuples().enumerate() {
                let mut each_child = |child: &str| -> usize {
                    let id = arena.add_new(
                        match child {
                            "G" | "H" => Kind::Goal,
                            "N" | "O" => Kind::Num,
                            _ => panic!("invalid char"),
                        },
                        Some(nodes[i]),
                    );
                    if child == "H" || child == "O" {
                        new_nodes.push(id);
                    }
                    id
                };

                arena.get_mut(nodes[i]).link = Link::Branch(each_child(left), each_child(right));
            }
            nodes = new_nodes;
        }
        arena
    }
    pub fn perm_map(&self) -> Vec<bool> {
        let mut map = Vec::new();
        for (id, node) in self.nodes.iter().enumerate() {
            if let Node {
                kind: Kind::Num,
                link: Link::Leaf,
                ..
            } = node
            {
                // get parent
                if let Some(parent_id) = node.parent {
                    let parent = self.get(parent_id);
                    let Link::Branch(left, _) = parent.link else { unreachable!() };
                    if left == id {
                        map.push(false);
                    } else {
                        map.push(true);
                    }
                } else {
                    map.push(true);
                }
            }
        }
        map
    }
    pub fn populate(&mut self, nums: &[f64], goal: Option<f64>, memo: &Memo) {
        self.keys = vec!["".to_string(); self.nodes.len()];
        for (i, (id, _)) in self
            .nodes
            .iter_mut()
            .enumerate()
            .filter(|(_, node)| {
                matches! {node, Node {
                    kind: Kind::Num,
                    link: Link::Leaf,
                    ..
                }}
            })
            .enumerate()
        {
            self.keys[id] = format!("N {}", nums[i]);
        }
        if let Some(goal) = goal {
            let goal_id = self.get_goal_id();
            self.keys[goal_id] = format!("G {}", goal);
        }
        self.keys = (0..self.nodes.len())
            .map(|id| self.init_node_key(id, memo))
            .collect();
    }
    fn init_node_key(&self, id: usize, memo: &Memo) -> String {
        let node = self.get(id);
        match node.link {
            Link::Leaf => {
                // assume it has been populated
                self.keys[id].clone()
            }
            Link::Branch(left, right) => {
                let left_key = self.init_node_key(left, memo);
                let right_key = self.init_node_key(right, memo);
                let kind = match node.kind {
                    Kind::Num => "N",
                    Kind::Goal => "G",
                };
                format!("{} ({} {})", kind, left_key, right_key)
            }
        }
    }
    pub fn get_goal_id(&self) -> usize {
        self.nodes
            .iter()
            .position(|node| matches!(node.kind, Kind::Goal) && matches!(node.link, Link::Leaf))
            .expect("tree has no goal")
    }
    pub fn get_vals_from_memo<'a>(&self, id: usize, memo: &'a Memo) -> &'a [Val] {
        let key = &self.keys[id];
        assert!(key.len() > 0);
        if let Some(vals) = memo.get(key) {
            vals
        } else {
            panic!("key not found in memo")
        }
    }

    pub fn set_vals_in_memo(&self, id: usize, vals: Vec<Val>, memo: &mut Memo) {
        let key = &self.keys[id];
        memo.insert(key.clone(), vals);
    }
    #[inline(never)]
    pub fn solve(&self, depth: usize, memo: &mut Memo) {
        fn rec(arena: &Arena, id: usize, depth: usize, memo: &mut Memo) {
            let node = arena.get(id);
            // check if in memo
            if let Some(_) = memo.get(&arena.keys[id]) {
                // if this node is calculated, children must be calculated
                return;
            }
            let mut vals = Vec::new();
            if let Link::Branch(left_id, right_id) = node.link {
                rec(arena, left_id, depth, memo);
                rec(arena, right_id, depth, memo);
                vals.extend(expand_node(arena, left_id, right_id, memo));
            }
            for i in 0..vals.len() {
                let val = vals[i].clone();
                vals.extend(
                    expand_funcs(val.num, matches!(node.kind, Kind::Goal), depth)
                        .into_iter()
                        .map(|(num, funcs)| val.clone_with_funcs(num, funcs)),
                );
            }
            arena.set_vals_in_memo(id, vals, memo);
        }
        rec(self, 0, depth, memo);
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn count_num_leaves(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| {
                matches!(
                    node,
                    Node {
                        kind: Kind::Num,
                        link: Link::Leaf,
                        ..
                    }
                )
            })
            .count()
    }
}
#[inline(never)]
pub fn expand_funcs(start: f64, reverse: bool, depth: usize) -> Vec<(f64, FuncList)> {
    let mut paths: Vec<(f64, FuncList)> = vec![(start, FuncList::new())];
    let mut high_paths_start = 0;

    for _ in 0..=depth {
        let new_paths: Vec<_> = paths[high_paths_start..]
            .iter()
            .filter(|(num, _)| num.fract() == 0.0) // TODO remove this
            .flat_map(|(num, funcs)| {
                Func::iter().filter_map(|func| {
                    func.apply_rev_if(*num, reverse).map(|num| {
                        let mut new_funcs = funcs.clone();
                        new_funcs.push(func);
                        (num, new_funcs)
                    })
                })
            })
            .collect();
        if new_paths.len() == 0 {
            break;
        }
        high_paths_start = paths.len();
        paths.extend(new_paths);
    }
    paths
}

#[inline(never)]
fn expand_node<'a>(
    arena: &'a Arena,
    left_id: usize,
    right_id: usize,
    memo: &'a Memo,
) -> impl Iterator<Item = Val> + 'a {
    let left_node = arena.get(left_id);
    let right_node = arena.get(right_id);

    iproduct!(
        arena.get_vals_from_memo(left_id, memo).iter().enumerate(),
        arena.get_vals_from_memo(right_id, memo).iter().enumerate()
    )
    .flat_map(|((left_i, left), (right_i, right))| {
        match (&left_node.kind, &right_node.kind) {
            (Kind::Num, Kind::Num) => Operation::apply_all(left.num, right.num, false),
            (Kind::Num, Kind::Goal) => Operation::apply_all(left.num, right.num, true),
            (Kind::Goal, Kind::Num) => Operation::apply_all(right.num, left.num, true),
            _ => panic!("two goals in one tree"),
        }
        .into_iter()
        .map(move |(op, num)| Val {
            num,
            origin: num,
            score: left.score + right.score + op.score(),
            funcs: FuncList::new(),
            path: Path::Combine {
                left: left_i,
                right: right_i,
                op,
            },
        })
    })
}

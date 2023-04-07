pub mod atom;
pub mod func;
pub mod func_list;
pub mod joiner;
pub mod math;
pub mod operation;
pub mod tree;

use joiner::create_base_trees;

pub fn solve(nums: Vec<f64>, goal: f64) {
    let joiners = create_base_trees();
    let mut max_score = 0;
    for (i, mut joiner) in joiners.into_iter().enumerate() {
        println!("started tree: {:?}", i);
        joiner.solve(&nums, goal, 5, &mut max_score);
    }
}

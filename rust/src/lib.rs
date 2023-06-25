use dateo_fun_remover::finder::solve as finder_solve;
use dateo_fun_remover::inputs::get_goal_and_nums;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve_with_date(year: u32, month: u32, day: u32) {
    let (goal, nums) = get_goal_and_nums(None, None, None, Some(day), Some(month), Some(year));
    finder_solve(&nums, goal, |score, atom| {
        sendNextSolution(format!("{} {}", score, atom));
    })
}
#[wasm_bindgen]
pub fn solve_with_goal_and_nums(
    goal: f64,
    num_1: f64,
    num_2: f64,
    num_3: f64,
    num_4: f64,
    num_5: f64,
) {
    let nums = vec![num_1, num_2, num_3, num_4, num_5];
    finder_solve(&nums, goal, |score, atom| {
        sendNextSolution(format!("{} {}", score, atom));
    })
}

#[wasm_bindgen]
extern "C" {
    pub fn sendNextSolution(solution: String); // Importing the JS-Function
}

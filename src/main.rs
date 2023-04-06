mod finder;
use finder::solve;
mod inputs;
use inputs::get_goal_and_nums;

fn main() {
    let (goal, nums) = get_goal_and_nums();

    println!("goal: {}", goal);
    // print space separated list of numbers
    println!(
        "nums: {}",
        nums.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    solve(nums, goal);
}

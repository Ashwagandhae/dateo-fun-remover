mod finder;
use finder::solve;
mod inputs;
use inputs::get_goal_and_nums_from_args;

fn main() {
    let (goal, nums) = get_goal_and_nums_from_args();

    println!("goal: {}", goal);
    // print space separated list of numbers
    println!(
        "nums: {}",
        nums.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    solve(&nums, goal, |score, atom| {
        println!("atom with score {}: {}", score, atom);
        atom.eval_verbose();
    })
}

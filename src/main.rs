mod finder;
use finder::{create_atom_store, create_goal_paths, get_solution_with_score};
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

    println!("loading atoms...");
    let store = create_atom_store(&nums);
    println!("atoms: {}", store.len());

    println!("loading goal paths...");
    let goal_paths = create_goal_paths(goal);
    goal_paths.print_list();

    let mut score = 0;
    loop {
        println!("searching for solutions with score {}", score);
        let ret = get_solution_with_score(score, &goal_paths, &store);
        if let Some((solution, delta)) = ret {
            if delta > 0 {
                println!("jumped {} points", delta);
            }
            score += delta;
            println!("found solution with score {}: {}", score, solution);
            solution.eval_verbose();
        }
        score += 1;
    }
}

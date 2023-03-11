mod finder;
use clap::Parser;
use finder::{create_atom_store, create_goal_paths, get_solution_with_score};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Given numbers
    #[arg(short, long, default_value = "-8 5 8 18 19")]
    nums: String,
    // Goal number
    #[arg(short, long, default_value = "6")]
    goal: u32,
}

fn main() {
    // read args
    let args: Args = Args::parse();
    let nums: Vec<f64> = args
        .nums
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let goal: f64 = args.goal as f64;

    println!("goal: {}", goal);
    // print space separated list of numbers
    println!(
        "nums: {}",
        nums.clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    println!("loading atoms...");
    let store = create_atom_store(&nums);
    println!("atoms: {}", store.len());

    println!("loading goal paths...");
    let goal_paths = create_goal_paths(goal, 16);
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

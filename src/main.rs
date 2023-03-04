mod finder;
use finder::{create_atom_store, create_goal_paths, get_solution_with_score};

fn main() {
    let nums: Vec<f64> = vec![-17., -11., 11., 12., 17.];
    let goal: f64 = 4.;

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

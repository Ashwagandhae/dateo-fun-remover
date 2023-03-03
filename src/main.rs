mod finder;
use finder::{create_atom_store, get_solution_with_score};

fn main() {
    let nums: Vec<f64> = vec![-16., -10., 2., 13., 16.];
    let goal: f64 = 19.;

    println!("loading atoms...");
    let store = create_atom_store(&nums);
    println!("atoms: {}", store.len());
    for score in 0..u32::MAX {
        println!("searching for solutions with score {}", score);
        let solution = get_solution_with_score(score, goal, &store);
        if let Some(solution) = solution {
            println!("found solution with score {}: {}", score, solution);
            solution.eval_verbose();
        }
    }
}

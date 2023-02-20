mod finder;
use finder::Finder;

fn main() {
    let nums: Vec<f64> = vec![-16., -10., 2., 13., 16.];
    // let nums: Vec<f64> = vec![-16., -10., -2.];
    let goal: f64 = 19.;

    let finder = Finder::new(nums, goal);
    finder.find_solutions(20);
}

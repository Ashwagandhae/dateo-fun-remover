use chrono::Datelike;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Given numbers (prioritized over date generated numbers). Input as space separated list of numbers, e.g. '1 2 3 4 5'
    #[arg(short, long, allow_hyphen_values = true)]
    nums: Option<String>,
    /// Goal number (prioritized over date generated numbers)
    #[arg(short, long, allow_hyphen_values = true)]
    goal: Option<f64>,
    /// Day of month to use for generating numbers
    #[arg(short, long)]
    day: Option<u32>,
    /// Month of year to use for generating numbers
    #[arg(short, long)]
    month: Option<u32>,
    /// Year to use for generating numbers
    #[arg(short, long)]
    year: Option<u32>,
}

type YearMonthDay = (u32, u32, u32);
fn get_current_date() -> YearMonthDay {
    let now = chrono::Local::now();
    (now.year() as u32, now.month(), now.day())
}

fn parse_nums(nums: &str) -> Vec<f64> {
    let ret: Vec<_> = nums.split(" ")
        .map(|num| num.parse::<f64>().expect("Number parsing error. Please provide a space separated list of numbers, e.g. '1 2 3 4 5'"))
        .collect();
    if ret.len() > 5 {
        panic!("Please provide no more than 5 numbers")
    }
    if ret.len() < 5 {
        panic!("Please provide at least 5 numbers")
    }
    ret
}

// based on https://dateo-math-game.com/setNumbers.js

fn guess_goal(date: YearMonthDay) -> f64 {
    date.2 as f64
}

fn rng(n: f64) -> f64 {
    (n * 7_f64.powf(5.)) % (2_f64.powf(31.) - 1.)
}
fn guess_nums(date: YearMonthDay) -> Vec<f64> {
    let mut seed = (date.2 as f64) + (100. * date.0 as f64) + (1_000_000. * date.1 as f64);
    let mut nums = Vec::new();
    let mut num;
    while nums.len() < 5 {
        seed = rng(seed);
        num = seed % 20. + 1.;
        if seed % 3. == 0. {
            num = -num;
        }
        if !nums.contains(&num) {
            nums.push(num);
        }
    }
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    nums
}

pub fn get_goal_and_nums_from_args() -> (f64, Vec<f64>) {
    let args = Args::parse();
    get_goal_and_nums(args.nums, args.goal, args.day, args.month, args.year)
}

pub fn get_goal_and_nums(
    nums: Option<String>,
    goal: Option<f64>,
    day: Option<u32>,
    month: Option<u32>,
    year: Option<u32>,
) -> (f64, Vec<f64>) {
    let mut date = match (year, month, day) {
        (Some(year), Some(month), Some(day)) => (year, month, day),
        _ => {
            let (real_year, real_month, real_day) = get_current_date();
            (
                year.unwrap_or(real_year),
                month.unwrap_or(real_month),
                day.unwrap_or(real_day),
            )
        }
    };
    // month is zero indexed in javascript
    date.1 -= 1;

    let goal = goal.unwrap_or(guess_goal(date));
    let nums = nums.map_or_else(|| guess_nums(date), |nums| parse_nums(&nums));
    (goal, nums)
}

use chrono::Datelike;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Given numbers (prioritized over date generated numbers)
    #[arg(short, long, allow_hyphen_values = true)]
    nums: Option<String>,
    /// Goal number (prioritized over date generated numbers)
    #[arg(short, long, allow_hyphen_values = true)]
    goal: Option<f64>,
    /// Full date to use for generating numbers
    #[arg(short, long)]
    full_date: Option<String>,
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
    // month is zero indexed in javascript
    (now.year() as u32, now.month() - 1, now.day())
}

fn parse_date(date: &str) -> YearMonthDay {
    let date = date.split("-").collect::<Vec<&str>>();
    (
        date[0].parse::<u32>().unwrap(),
        date[1].parse::<u32>().unwrap(),
        date[2].parse::<u32>().unwrap(),
    )
}

fn parse_nums(nums: &str) -> Vec<f64> {
    nums.split(" ")
        .map(|num| num.parse::<f64>().unwrap())
        .collect()
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
    get_goal_and_nums(
        args.nums,
        args.goal,
        args.full_date,
        args.day,
        args.month,
        args.year,
    )
}

pub fn get_goal_and_nums(
    nums: Option<String>,
    goal: Option<f64>,
    full_date: Option<String>,
    day: Option<u32>,
    month: Option<u32>,
    year: Option<u32>,
) -> (f64, Vec<f64>) {
    let date = match full_date {
        Some(date) => parse_date(&date),
        // fill in missing date parts with current date
        None => match (year, month, day) {
            (Some(year), Some(month), Some(day)) => (year, month, day),
            _ => {
                let (year, month, day) = get_current_date();
                (year, month, day)
            }
        },
    };
    let goal = goal.unwrap_or(guess_goal(date));
    let nums = nums.map_or_else(|| guess_nums(date), |nums| parse_nums(&nums));
    (goal, nums)
}

use crate::utils::Problem;

mod day1;
mod day2;
pub mod utils;

fn main() {
    // TODO: maybe parse args for desired day?
    let days: Vec<Box<dyn Problem>> = vec![Box::new(day1::Day1 {}), Box::new(day2::Day2 {})];
    for day in days {
        println!("DAY {}: {}, {}", day.day(), day.part_one(), day.part_two())
    }
}

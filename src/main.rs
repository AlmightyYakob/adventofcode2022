use crate::utils::Problem;

mod day1;
mod day10;
mod day2;
mod day6;
mod day7;
mod day8;
pub mod utils;

fn main() {
    // TODO: maybe parse args for desired day?
    let days: Vec<Box<dyn Problem>> = vec![
        Box::new(day1::Day1 {}),
        Box::new(day2::Day2 {}),
        Box::new(day6::Day6 {}),
        Box::new(day7::Day7 {}),
        Box::new(day8::Day8 {}),
        Box::new(day10::Day10 {}),
    ];
    for day in days {
        println!("DAY {}: {}, {}", day.day(), day.part_one(), day.part_two())
    }
}

use std::cmp;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use super::utils::Problem;

fn get_input() -> BufReader<File> {
    let file = File::open("src/day7/input.txt").unwrap();

    BufReader::new(file)
}

pub struct DayX {}
impl Problem for DayX {
    fn day(&self) -> &str {
        return "X";
    }
}

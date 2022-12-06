use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use super::utils::Problem;

pub struct Day6 {}
impl Problem for Day6 {
    fn day(&self) -> &str {
        return "SIX";
    }

    fn part_one(&self) -> String {
        let file = File::open("src/day6/input.txt").unwrap();
        let reader = BufReader::new(file);

        // Get chars
        let line = reader
            .lines()
            .next()
            .expect("No lines to read")
            .expect("Error reading line");

        let mut set = HashSet::<char>::new();
        for i in 0..(line.len() - 3) {
            set.clear();

            let slice = &line[i..i + 4];
            set.extend(slice.chars());
            if set.len() == 4 {
                return (i + 4).to_string();
            }
        }

        return "NOT FOUND".to_string();
    }

    fn part_two(&self) -> String {
        let file = File::open("src/day6/input.txt").unwrap();
        let reader = BufReader::new(file);

        // Get chars
        let line = reader
            .lines()
            .next()
            .expect("No lines to read")
            .expect("Error reading line");

        let mut set = HashSet::<char>::new();
        for i in 0..(line.len() - 13) {
            set.clear();

            let slice = &line[i..i + 14];
            set.extend(slice.chars());
            if set.len() == 14 {
                return (i + 14).to_string();
            }
        }

        return "NOT FOUND".to_string();
    }
}

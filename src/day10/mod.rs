use std::cmp::{self, Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

use super::utils::Problem;

fn get_input() -> BufReader<File> {
    let file = File::open("src/day10/input.txt").unwrap();

    BufReader::new(file)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum INST {
    Noop,
    Addx(i32),
}
impl INST {
    pub fn runtime(&self) -> u32 {
        return match self {
            INST::Noop => 1,
            INST::Addx(_) => 2,
        };
    }
    pub fn from(input: &String) -> INST {
        let split: Vec<&str> = input.split_whitespace().collect();
        if split.len() == 2 {
            return INST::Addx(split[1].parse().unwrap());
        }

        return INST::Noop;
    }
}

#[derive(PartialEq, Eq)]
struct QueuedInst {
    inst: INST,
    exec_cycle: u32,
}
impl Ord for QueuedInst {
    fn cmp(&self, other: &Self) -> Ordering {
        self.exec_cycle.cmp(&other.exec_cycle).reverse()
    }
}

impl PartialOrd for QueuedInst {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day10 {}
impl Problem for Day10 {
    fn day(&self) -> &str {
        return "TEN";
    }

    fn part_one(&self) -> String {
        let mut checkpoint_sum = 0;
        let checkpoints: HashSet<u32> = (20..=220).step_by(40).collect();

        let mut cycle = 1u32;
        let mut register = 1i32;

        let reader = get_input();
        for line in reader.lines().filter_map(|x| x.ok()) {
            let inst = INST::from(&line);

            // Check register before processing
            let new_cycle = cycle + inst.runtime();
            for n in cycle..(new_cycle) {
                if checkpoints.contains(&n) {
                    checkpoint_sum += register * (n as i32);
                }
            }

            // Process inst
            cycle = new_cycle;
            match inst {
                INST::Addx(x) => {
                    register += x;
                }
                _ => {}
            }
        }
        checkpoint_sum.to_string()
    }

    fn part_two(&self) -> String {
        let linebreaks: HashSet<u32> = (40..=240).step_by(40).collect();
        let mut cycle = 1u32;
        let mut register = 1i32;

        let reader = get_input();
        for line in reader.lines().filter_map(|x| x.ok()) {
            let inst = INST::from(&line);

            // Compute bounds of 3 pixel wide thing
            let (low, high) = (cmp::max(register - 1, 0), cmp::min(register + 1, 39));

            // Process
            let new_cycle = cycle + inst.runtime();

            // for n in cycle..(new_cycle) {
            //     let pixel = ((n - 1) as i32) % 40;
            //     if low <= pixel && pixel <= high {
            //         print!("#");
            //     } else {
            //         print!(".")
            //     }
            //     if linebreaks.contains(&n) {
            //         print!("\n");
            //     }
            // }

            // Process inst
            cycle = new_cycle;
            match inst {
                INST::Addx(x) => {
                    register += x;
                }
                _ => {}
            }
        }

        String::new()
    }
}

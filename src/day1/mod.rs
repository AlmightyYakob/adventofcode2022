use std::cmp;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn find_max_calories() -> i32 {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current_max = 0;
    let mut current_sum = 0;
    for line in reader.lines() {
        let text = line.unwrap();
        if text == "" {
            current_max = cmp::max(current_max, current_sum);
            current_sum = 0;
            continue;
        }

        current_sum += text.parse::<i32>().unwrap();
    }

    return current_max;
}

pub fn find_top_three_max_calories() -> i32 {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut heap = BinaryHeap::new();
    let mut current_sum = 0;
    for line in reader.lines() {
        let text = line.unwrap();
        if text == "" {
            heap.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += text.parse::<i32>().unwrap();
        }
    }

    return heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
}

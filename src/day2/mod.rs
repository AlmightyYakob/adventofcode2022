use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use super::utils::Problem;

#[derive(PartialEq, Clone, Copy, Eq)]
enum Shape {
    Rock = 1,
    Paper,
    Scissors,
}
impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        // Paper covers rock
        if *self == Shape::Rock && *other == Shape::Paper {
            return Ordering::Less;
        }
        if *self == Shape::Paper && *other == Shape::Rock {
            return Ordering::Greater;
        }

        // Rock breaks scissors
        if *self == Shape::Scissors && *other == Shape::Rock {
            return Ordering::Less;
        }
        if *self == Shape::Rock && *other == Shape::Scissors {
            return Ordering::Greater;
        }

        // Scissors cut paper
        if *self == Shape::Paper && *other == Shape::Scissors {
            return Ordering::Less;
        }
        if *self == Shape::Scissors && *other == Shape::Paper {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    }
}
impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Shape {
    fn parse(input: &str) -> Shape {
        match input {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" | _ => Shape::Scissors,
        }
    }
    fn to_winner(&self) -> Shape {
        match *self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
    fn to_loser(&self) -> Shape {
        match *self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}
impl Outcome {
    fn parse(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" | _ => Outcome::Win,
        }
    }
}

pub struct Day2 {}
impl Problem for Day2 {
    fn day(&self) -> &str {
        "TWO"
    }

    fn part_one(&self) -> String {
        let file = File::open("src/day2/input.txt").unwrap();
        let reader = BufReader::new(file);

        let mut total_score = 0;
        for line in reader.lines() {
            let text = line.unwrap();
            let mut iter = text.split_whitespace();
            let (opponent, response) = (
                Shape::parse(iter.next().unwrap()),
                Shape::parse(iter.next().unwrap()),
            );

            // now lets get moving here
            let mut score = response as i32;
            if opponent == response {
                score += 3;
            } else if response > opponent {
                score += 6;
            }
            total_score += score;
        }

        return total_score.to_string();
    }

    fn part_two(&self) -> String {
        let file = File::open("src/day2/input.txt").unwrap();
        let reader = BufReader::new(file);

        let mut total_score = 0;
        for line in reader.lines() {
            let text = line.unwrap();
            let mut iter = text.split_whitespace();
            let (opponent, outcome) = (
                Shape::parse(iter.next().unwrap()),
                Outcome::parse(iter.next().unwrap()),
            );

            // Add outcome score
            let mut score = 0;
            match outcome {
                Outcome::Draw => {
                    score += 3;
                    score += opponent as i32;
                }
                Outcome::Win => {
                    score += 6;
                    score += opponent.to_winner() as i32;
                }
                Outcome::Loss => {
                    score += opponent.to_loser() as i32;
                }
            }
            total_score += score;
        }

        return total_score.to_string();
    }
}

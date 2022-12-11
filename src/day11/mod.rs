use num::bigint::BigUint;
use num::Zero;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

use super::utils::Problem;

fn get_input() -> BufReader<File> {
    let file = File::open("src/day11/input.txt").unwrap();

    BufReader::new(file)
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u32),
    Mult(u32),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<BigUint>,
    items_inspected: usize,
    operation: Operation,
    test_factor: u32,
    test_true_monkey: u32,
    test_false_monkey: u32,
}

fn parse_input() -> Vec<Monkey> {
    let reader = get_input();
    let lines: Vec<String> = reader.lines().filter_map(|x| x.ok()).collect();

    // Regexes
    let monkey_re = Regex::new(r"Monkey (\d+):").unwrap();
    let starting_items_re = Regex::new(r"Starting items: ((?:\d+(?:, )?)+)").unwrap();
    let operation_re = Regex::new(r"Operation: new = old ([*+]) (\d+|old)").unwrap();
    let test_re = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let monkey_pass_re = Regex::new(r"throw to monkey (\d+)").unwrap();

    // Iterate and parse
    let mut monkeys: Vec<Monkey> = vec![];
    for mut i in 0..lines.len() {
        let line = &lines[i];
        if monkey_re.is_match(line) {
            let starting_items_match = starting_items_re.captures(lines[i + 1].as_str()).unwrap();
            let items = starting_items_match
                .get(1)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();

            let op_match = operation_re.captures(lines[i + 2].as_str()).unwrap();
            let op_factor = op_match.get(2).unwrap().as_str().parse();
            let operation = match op_match.get(1).unwrap().as_str() {
                "*" => match op_factor {
                    Ok(x) => Operation::Mult(x),
                    Err(_) => Operation::Square,
                },
                "+" | _ => Operation::Add(op_factor.unwrap()),
            };

            let test_match = test_re.captures(lines[i + 3].as_str()).unwrap();
            let test_factor = test_match.get(1).unwrap().as_str().parse().unwrap();

            let pass_match_true = monkey_pass_re.captures(lines[i + 4].as_str()).unwrap();
            let test_true_monkey = pass_match_true.get(1).unwrap().as_str().parse().unwrap();

            let pass_match_false = monkey_pass_re.captures(lines[i + 5].as_str()).unwrap();
            let test_false_monkey = pass_match_false.get(1).unwrap().as_str().parse().unwrap();

            // Push monkey
            i += 5;
            monkeys.push(Monkey {
                items,
                items_inspected: 0,
                operation,
                test_factor,
                test_true_monkey,
                test_false_monkey,
            })
        }
    }
    return monkeys;
}

fn run_round(monkeys: &mut Vec<Monkey>, part_one: bool) {
    for i in 0..monkeys.len() {
        // Process all items
        while !monkeys[i].items.is_empty() {
            let item = monkeys[i].items.pop_front().unwrap();
            monkeys.get_mut(i).unwrap().items_inspected += 1;

            // Inspect
            let mut worry_level = item;
            // println!(
            //     "WORRY LEVEL {}, OP: {:?}",
            //     &worry_level, &monkeys[i].operation
            // );
            worry_level = match monkeys[i].operation {
                Operation::Add(x) => worry_level + x,
                Operation::Mult(x) => worry_level * x,
                Operation::Square => worry_level.pow(2),
            };
            if part_one {
                worry_level = worry_level / BigUint::from(3u8);
            }

            // Test
            let mut next_monkey_index = monkeys[i].test_false_monkey as usize;
            if &worry_level % monkeys[i].test_factor == BigUint::zero() {
                next_monkey_index = monkeys[i].test_true_monkey as usize;
                // worry_level = worry_level / monkeys[i].test_factor;
            }

            // Pass
            monkeys[next_monkey_index].items.push_back(worry_level);
        }
    }

    // test
    let mut items = vec![];
    for monkey in monkeys {
        items.extend(monkey.items.clone());
    }
    println!("ITEMS: {:?}", items);
}

pub struct Day11 {}
impl Problem for Day11 {
    fn day(&self) -> &str {
        return "ELEVEN";
    }

    fn part_one(&self) -> String {
        let mut monkeys = parse_input();
        for _ in 0..20 {
            run_round(&mut monkeys, true);
        }

        // Sort descending
        let mut sorted_monkeys: Vec<usize> = monkeys.iter().map(|m| m.items_inspected).collect();
        sorted_monkeys.sort_by(|a, b| b.cmp(a));

        let monkey_business = sorted_monkeys[0] * sorted_monkeys[1];
        return monkey_business.to_string();
    }
    fn part_two(&self) -> String {
        let mut monkeys = parse_input();
        for _ in 0..10 {
            run_round(&mut monkeys, false);
        }

        // Sort descending
        let mut sorted_monkeys: Vec<usize> = monkeys.iter().map(|m| m.items_inspected).collect();
        sorted_monkeys.sort_by(|a, b| b.cmp(a));

        let monkey_business = sorted_monkeys[0] * sorted_monkeys[1];
        return monkey_business.to_string();
    }
}

use std::fs::File;
use std::io::{prelude::*, BufReader};

use super::utils::Problem;

fn get_input() -> BufReader<File> {
    let file = File::open("src/day8/input.txt").unwrap();

    BufReader::new(file)
}

pub struct Day8 {}
impl Problem for Day8 {
    fn day(&self) -> &str {
        return "EIGHT";
    }

    fn part_one(&self) -> String {
        let input = get_input();
        let mut matrix: Vec<Vec<u8>> = vec![];
        let mut visible: Vec<Vec<bool>> = vec![];

        let lines: Vec<String> = input.lines().filter_map(|l| l.ok()).collect();
        for (row, line) in lines.iter().enumerate() {
            let mut data = vec![];
            let mut marks = vec![];
            for (col, c) in line.chars().enumerate() {
                let num = c.to_digit(10).unwrap() as u8;
                data.push(num);

                // Mark outside rim visible or not
                if (row == 0 || row == lines.len() - 1) || (col == 0 || col == line.len() - 1) {
                    marks.push(true)
                } else {
                    marks.push(false)
                }
            }

            matrix.push(data);
            visible.push(marks);
        }

        // The number of counted visible trees
        // Start with num from circumference
        let mut num_visible = matrix.len().pow(2) - (matrix.len() - 2).pow(2);

        // Now let's actually do it
        // Approach each row/line from all four directions.
        // Any time a tree strictly increases in value, mark it visible

        // Left/Right
        for (row, row_data) in matrix.iter().enumerate() {
            let mut height = 0;
            for (col, x) in row_data.iter().enumerate() {
                if *x > height {
                    if !visible[row][col] {
                        num_visible += 1;
                        visible[row][col] = true;
                    }

                    height = *x;
                }
            }

            height = 0;
            for (col, x) in row_data.iter().enumerate().rev() {
                if *x > height {
                    if !visible[row][col] {
                        num_visible += 1;
                        visible[row][col] = true;
                    }

                    height = *x;
                }
            }
        }

        // Up/Down
        for col in 0..matrix[0].len() {
            let mut height = 0;

            // Down
            for row in 0..matrix.len() {
                if matrix[row][col] > height {
                    if !visible[row][col] {
                        num_visible += 1;
                        visible[row][col] = true;
                    }

                    height = matrix[row][col];
                }
            }

            // Up
            height = 0;
            for row in (0..matrix.len()).rev() {
                if matrix[row][col] > height {
                    if !visible[row][col] {
                        num_visible += 1;
                        visible[row][col] = true;
                    }

                    height = matrix[row][col];
                }
            }
        }

        num_visible.to_string()
    }
}

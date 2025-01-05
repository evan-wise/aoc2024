use crate::aoc::{read_lines, Answers, Solution};
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Day02 {
    reports: Vec<Vec<i32>>,
}

impl Day02 {
    pub fn new() -> Day02 {
        Day02 {
            reports: Vec::new(),
        }
    }
}

impl Solution for Day02 {
    type Part1 = i32;
    type Part2 = i32;

    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day02.txt")?;
        for line in lines.flatten() {
            self.reports.push(
                line.split(" ")
                    .map(|n| n.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers<Self::Part1, Self::Part2>, Box<dyn Error>> {
        let mut count = 0;
        let mut dampener_count = 0;
        for nums in &self.reports {
            if check_safety(&nums) {
                count += 1;
                dampener_count += 1;
            } else {
                for i in 0..nums.len() {
                    let reduced_nums = [&nums[..i], &nums[i + 1..]].concat();
                    if check_safety(&reduced_nums) {
                        dampener_count += 1;
                        break;
                    }
                }
            }
        }
        Answers::ok(Some(count), Some(dampener_count))
    }
}

fn parse_line(line: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut nums = Vec::new();
    for raw in line.split(" ") {
        let num = raw.parse::<i32>()?;
        nums.push(num);
    }
    Ok(nums)
}

fn check_safety(nums: &Vec<i32>) -> bool {
    let mut is_safe = true;
    let mut maybe_prev = None;
    let mut maybe_dir = None;
    for num in nums.iter() {
        if let Some(prev) = maybe_prev {
            let diff: i32 = num - prev;
            let abs = diff.abs();
            if abs < 1 || abs > 3 {
                is_safe = false;
                break;
            }
            if let Some(dir) = maybe_dir {
                if dir != diff.signum() {
                    is_safe = false;
                    break;
                }
            } else {
                maybe_dir = Some(diff.signum())
            }
        }
        maybe_prev = Some(num);
    }
    is_safe
}

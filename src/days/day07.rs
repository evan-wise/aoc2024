use crate::aoc::{read_lines, Solution, SolutionParts};
use std::error::Error;

pub struct Day07;

impl Solution for Day07 {
    fn solve(&self) -> Result<SolutionParts, Box<dyn Error>> {
        let mut total = 0;
        let mut total_with_concat = 0;
        let lines = read_lines("./data/day07.txt")?;
        for line in lines.flatten() {
            let chunks: Vec<&str> = line.split(": ").collect();
            if chunks.len() != 2 {
                return Err("invalid line".into());
            }
            let test_val = chunks[0].parse::<u64>()?;
            let nums: Vec<u64> = chunks[1]
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            if check_equation(test_val, &nums, false) {
                total += test_val;
            }
            if check_equation(test_val, &nums, true) {
                total_with_concat += test_val;
            }
        }
        Ok((Some(total.to_string()), Some(total_with_concat.to_string())))
    }
}

fn check_equation(test_val: u64, nums: &[u64], allow_concat: bool) -> bool {
    if nums.len() == 0 {
        return false;
    }

    if nums.len() == 1 {
        return test_val == nums[0];
    }

    let (last, rest) = nums.split_last().unwrap();

    if test_val % last == 0 {
        if check_equation(test_val / last, rest, allow_concat) {
            return true;
        }
    }

    if allow_concat {
        let test_str = test_val.to_string();
        let last_str = last.to_string();
        if test_str.ends_with(&last_str) {
            let new_test_val = test_str
                .strip_suffix(&last_str)
                .unwrap()
                .parse::<u64>()
                .unwrap_or(0);
            if check_equation(new_test_val, rest, allow_concat) {
                return true;
            }
        }
    }

    if check_equation(test_val.saturating_sub(*last), rest, allow_concat) {
        return true;
    }

    false
}

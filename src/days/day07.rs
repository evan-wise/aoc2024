use crate::aoc::{read_lines, Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day07 {
    equations: Vec<(u64, Vec<u64>)>,
}

impl Day07 {
    pub fn new() -> Day07 {
        Day07 {
            equations: Vec::new(),
        }
    }
}

impl Solution for Day07 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day07.txt")?;
        for line in lines.flatten() {
            let chunks: Vec<&str> = line.split(": ").collect();
            if chunks.len() != 2 {
                return Err("invalid line".into());
            }
            let test_val = chunks[0].parse::<u64>()?;
            let nums: Vec<u64> = chunks[1]
                .split(" ")
                .map(|s| s.parse::<u64>())
                .collect::<Result<_, _>>()?;
            self.equations.push((test_val, nums));
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut total = 0;
        let mut total_with_concat = 0;
        for (test_val, nums) in &self.equations {
            if check_equation(*test_val, &nums, false) {
                total += test_val;
            }
            if check_equation(*test_val, &nums, true) {
                total_with_concat += test_val;
            }
        }
        Ok(Answers::both(total, total_with_concat))
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

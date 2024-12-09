use crate::aoc::read_lines;
use crate::days::Solution;
use std::error::Error;
use std::num::ParseIntError;

pub struct Day02;

impl Solution for Day02 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        let mut count = 0;
        let mut dampener_count = 0;
        if let Ok(lines) = read_lines("./data/day02.txt") {
            for line in lines.flatten() {
                let nums = parse_line(&line)?;
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
        }
        println!("There are {} safe reports.", count);
        println!(
            "There are {} safe reports with the problem dampener.",
            dampener_count
        );
        Ok(())
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

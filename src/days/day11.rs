use crate::aoc::Solution;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::num::ParseIntError;

pub struct Day11;

impl Solution for Day11 {
    fn solve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let raw = read_to_string("./data/day11.txt")?;
        let stones: Vec<u64> = raw
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut stone_count_by_num = HashMap::new();
        for &stone in &stones {
            if let Some(&count) = stone_count_by_num.get(&stone) {
                stone_count_by_num.insert(stone, count + 1);
            } else {
                stone_count_by_num.insert(stone, 1 as u64);
            }
        }
        for _ in 0..25 {
            stone_count_by_num = blink(&stone_count_by_num)?;
        }
        println!(
            "There are {} stone(s) after 25 blinks",
            get_total(&stone_count_by_num)
        );
        for _ in 0..50 {
            stone_count_by_num = blink(&stone_count_by_num)?;
        }
        println!(
            "There are {} stone(s) after 75 blinks",
            get_total(&stone_count_by_num)
        );
        Ok(())
    }
}

fn get_total(stone_count_by_num: &HashMap<u64, u64>) -> u64 {
    let mut total = 0;
    for (_, &count) in stone_count_by_num {
        total += count;
    }
    total
}

fn blink(stone_count_by_num: &HashMap<u64, u64>) -> Result<HashMap<u64, u64>, ParseIntError> {
    let mut new = HashMap::new();
    for (&stone, &stone_count) in stone_count_by_num {
        let new_stones = match stone {
            0 => vec![1],
            _ => {
                let stone_str = stone.to_string();
                let stone_len = stone_str.len();
                if stone_len % 2 == 0 {
                    let half = (stone_len / 2) as u32;
                    let divisor = (10 as u64).pow(half);
                    vec![stone / divisor, stone % divisor]
                } else {
                    vec![2024 * stone]
                }
            }
        };
        for &new_stone in &new_stones {
            if let Some(&count) = new.get(&new_stone) {
                new.insert(new_stone, stone_count + count);
            } else {
                new.insert(new_stone, stone_count);
            }
        }
    }
    Ok(new)
}

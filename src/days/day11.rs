use crate::aoc::{Answers, Solution};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Day11 {
    stone_count_by_num: HashMap<u64, u64>,
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 {
            stone_count_by_num: HashMap::new(),
        }
    }
}

impl Solution for Day11 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let raw = read_to_string("./data/day11.txt")?;
        for stone in raw.split_whitespace().map(|s| s.parse::<u64>()).flatten() {
            *self.stone_count_by_num.entry(stone).or_insert(0) += 1;
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        for _ in 0..25 {
            self.stone_count_by_num = blink(&self.stone_count_by_num)?;
        }
        let total1 = get_total(&self.stone_count_by_num);
        for _ in 0..50 {
            self.stone_count_by_num = blink(&self.stone_count_by_num)?;
        }
        let total2 = get_total(&self.stone_count_by_num);
        Ok(Answers::from(Some(total1), Some(total2)))
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

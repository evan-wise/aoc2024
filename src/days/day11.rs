use crate::days::Solution;
use std::fs::read_to_string;
use std::num::ParseIntError;

pub struct Day11;

impl Solution for Day11 {
    fn solve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let raw = read_to_string("./data/day11.txt")?;
        let mut stones: Vec<u64> = raw.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
        for _ in 0..25 {
            blink(&mut stones)?;
        }
        println!("There are {} stone(s) after 25 blinks", stones.len());
        Ok(())
    }
}

fn blink(stones: &mut Vec<u64>) -> Result<(), ParseIntError> {
    let mut num_stones = stones.len();
    let mut i = 0;
    while i < num_stones {
        match stones[i] {
            0 => {
                stones[i] = 1;
                i += 1;
            },
            _ => {
                let stone_str = stones[i].to_string();
                let stone_len = stone_str.len();
                if stone_len % 2 == 0 {
                    let half = stone_len / 2;
                    let first = stone_str[0..half].parse::<u64>()?;
                    let second = stone_str[half..stone_len].parse::<u64>()?;
                    stones.splice(i..=i, [first, second]);
                    i += 2;
                    num_stones += 1;
                } else {
                    stones[i] = 2024 * stones[i];
                    i += 1;
                }
            }
        }
    }
    Ok(())
}

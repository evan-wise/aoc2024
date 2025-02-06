use crate::aoc::{Answers, Solution};
use std::{error::Error, fs::read_to_string};

#[derive(Debug)]
pub struct Day25 {
    locks: Vec<Levels>,
    keys: Vec<Levels>,
}

impl Day25 {
    pub fn new() -> Day25 {
        Day25 {
            locks: Vec::new(),
            keys: Vec::new(),
        }
    }
}

type Levels = [u8; 5];

impl Solution for Day25 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day25.txt";
        let raw = read_to_string(filename)?;
        let parts = raw.split("\n\n");
        for part in parts {
            if part.starts_with("#") {
                let mut levels = [0; 5];
                let lines = part.split("\n");
                for line in lines {
                    if line == "" { continue; }
                    for (i, c) in line.chars().enumerate() {
                        if c == '#' {
                            levels[i] += 1;
                        }
                    }
                }
                self.locks.push(levels);
            }
            if part.starts_with(".") {
                let mut levels = [7; 5];
                let lines = part.split("\n");
                for line in lines {
                    if line == "" { continue; }
                    for (i, c) in line.chars().enumerate() {
                        if c == '.' {
                            levels[i] -= 1;
                        }
                    }
                }
                self.keys.push(levels);
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut part1 = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if possible_fit(lock, key) {
                    part1 += 1;
                }
            }
        }
        Ok(Answers::both(part1, "Yay!"))
    }
}

fn possible_fit(lock: &Levels, key: &Levels) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 7 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day25::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(3065, "Yay!"));
        Ok(())
    }
}

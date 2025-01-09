use crate::aoc::{Answers, Solution};
use std::error::Error;
use std::fs::read_to_string;
use rustc_hash::FxHashSet;

pub struct Day19 {
    available: FxHashSet<String>,
    patterns: Vec<String>,
}

impl Day19 {
    pub fn new() -> Day19 {
        Day19 {
            available: FxHashSet::default(),
            patterns: Vec::new(),
        }
    }
}

impl Solution for Day19 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let raw = read_to_string("./data/day19.txt")?;
        let parts = raw.split("\n\n").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err("invalid input".into());
        }
        self.available
            .extend(parts[0].split(", ").map(|t| t.to_string()));
        self.patterns
            .extend(parts[1].split("\n").filter(|p| *p != "").map(|p| p.to_string()));
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut count = 0;
        for pattern in &self.patterns {
            if find_recipe(pattern, &self.available) {
                count += 1;
            }
        }
        Ok(Answers::from::<_, String>(Some(count), None))
    }
}

fn find_recipe(pattern: &str, available: &FxHashSet<String>) -> bool {
    let mut stack: Vec<(&str, &str)> = available.iter().map(|t| (&pattern[..], &t[..])).collect();
    if pattern == "" {
        return true;
    }
    while let Some((pattern, towel)) = stack.pop() {
        let l = towel.len();
        if pattern.len() == l {
            if pattern == towel {
                return true;
            }
        } else if pattern.len() > l {
            let prefix = &pattern[0..l];
            let reduced = &pattern[l..];
            if prefix == towel {
                stack.extend(available.iter().map(|t| (reduced, &t[..])));
            }
        }
    }
    false
}

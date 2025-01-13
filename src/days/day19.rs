use crate::aoc::{Answers, Solution};
use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
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
        self.patterns.extend(
            parts[1]
                .split("\n")
                .filter(|p| *p != "")
                .map(|p| p.to_string()),
        );
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut num_possible = 0;
        let mut num_ways = 0;
        for pattern in &self.patterns {
            let mut memos = FxHashMap::default();
            let recipes = count_recipes(pattern, &self.available, &mut memos);
            if recipes > 0 {
                num_possible += 1;
            }
            num_ways += recipes;
        }
        Ok(Answers::both(num_possible, num_ways))
    }
}

fn count_recipes(
    pattern: &str,
    available: &FxHashSet<String>,
    memos: &mut FxHashMap<String, usize>,
) -> usize {
    if pattern == "" {
        return 1;
    }
    if let Some(c) = memos.get(pattern) {
        return *c;
    }
    for towel in available {
        let l = towel[..].len();
        if pattern.len() < l {
            continue;
        }
        let prefix = &pattern[0..l];
        let reduced = &pattern[l..];
        if prefix == towel {
            *memos.entry(pattern.to_string()).or_insert(0) +=
                count_recipes(reduced, available, memos);
        }
    }
    *memos.get(pattern).unwrap_or(&0)
}

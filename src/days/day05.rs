use crate::aoc::{read_lines, Answers, Solution};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct Day05 {
    rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

impl Day05 {
    pub fn new() -> Self {
        Day05 {
            rules: HashMap::new(),
            updates: Vec::new(),
        }
    }
}

impl Solution for Day05 {
    type Part1 = usize;
    type Part2 = usize;

    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day05.txt")?;
        let mut in_rules_section = true;
        for line in lines.flatten() {
            if &line == "" {
                in_rules_section = false;
            } else if in_rules_section {
                let values = line
                    .split('|')
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.len() < 2 {
                    return Err("invalid rule".into());
                }
                self.rules
                    .entry(values[0])
                    .or_insert_with(Vec::new)
                    .push(values[1]);
            } else {
                self.updates.push(
                    line.split(',')
                        .map(|s| s.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()?,
                );
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers<Self::Part1, Self::Part1>, Box<dyn Error>> {
        let mut correct_total = 0;
        let mut corrected_total = 0;
        for pages in &mut self.updates {
            let num_pages = pages.len();
            let mut is_correct = true;
            for i in 0..num_pages {
                let page = pages[i];
                for j in i + 1..num_pages {
                    let other = pages[j];
                    let afters = self.rules.get(&other).ok_or("failed to retrieve")?;
                    if afters.contains(&page) {
                        is_correct = false;
                        break;
                    }
                }
                if !is_correct {
                    break;
                }
            }
            if is_correct {
                correct_total += pages[num_pages / 2];
            } else {
                // Consider changing solve to mutable for better performance?
                pages.sort_by(|a, b| {
                    let default = &Vec::new();
                    let afters_a = self.rules.get(&a).unwrap_or(default);
                    let afters_b = self.rules.get(&b).unwrap_or(default);
                    if afters_a.contains(&b) {
                        return Ordering::Less;
                    }
                    if afters_b.contains(&a) {
                        return Ordering::Greater;
                    }
                    Ordering::Equal
                });

                corrected_total += pages[num_pages / 2];
            }
        }
        Answers::ok(Some(correct_total), Some(corrected_total))
    }
}

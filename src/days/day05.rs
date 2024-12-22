use crate::aoc::{read_lines, Solution};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

pub struct Day05;

impl Solution for Day05 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        let mut rules: HashMap<String, Vec<String>> = HashMap::new();
        let mut correct_total = 0;
        let mut corrected_total = 0;
        let mut in_rules_section = true;
        if let Ok(lines) = read_lines("./data/day05.txt") {
            for line in lines.flatten() {
                if &line == "" {
                    in_rules_section = false;
                } else if in_rules_section {
                    let values: Vec<String> = line.split('|').map(|s| s.to_string()).collect();
                    if values.len() < 2 {
                        return Err("invalid rule".into());
                    }
                    let before = values[0].clone();
                    let after = values[1].clone();
                    if let Some(afters) = rules.get_mut(&before) {
                        afters.push(after);
                    } else {
                        rules.insert(before, vec![after]);
                    }
                } else {
                    let mut values: Vec<&str> = line.split(',').collect();
                    let num_values = values.len();
                    let mut is_correct = true;
                    for i in 0..num_values {
                        let value = values[i].to_string();
                        for j in i + 1..num_values {
                            let other = values[j];
                            let afters = rules.get(other).ok_or("failed to retrieve")?;
                            if afters.contains(&value) {
                                is_correct = false;
                                break;
                            }
                        }
                        if !is_correct {
                            break;
                        }
                    }
                    if is_correct {
                        correct_total += values[num_values / 2].parse::<i32>()?;
                    } else {
                        // This feels really wonky but I am too much of a noob to sort it out at the
                        // moment.
                        values.sort_by(|a, b| {
                            let default = &Vec::new();
                            let afters_a = rules.get(*a).unwrap_or(default);
                            let afters_b = rules.get(*b).unwrap_or(default);
                            if afters_a.contains(&(*b).to_string()) {
                                return Ordering::Less;
                            }
                            if afters_b.contains(&(*a).to_string()) {
                                return Ordering::Greater;
                            }
                            Ordering::Equal
                        });

                        corrected_total += values[num_values / 2].parse::<i32>()?;
                    }
                }
            }
        }
        println!(
            "The sum of the middle values of the already correct updates is: {}",
            correct_total
        );
        println!(
            "The sum of the middle values of the corrected updates is: {}",
            corrected_total
        );
        Ok(())
    }
}

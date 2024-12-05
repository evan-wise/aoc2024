use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut total = 0;
    let mut in_rules_section = true;
    if let Ok(lines) = read_lines("./data/rules_and_updates.txt") {
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
                let values: Vec<&str> = line.split(',').collect();
                let num_values = values.len();
                let mut middle = 0; 
                let mut is_correct = true;
                for i in 0..num_values {
                    let value = values[i].to_string();
                    if i == num_values / 2 {
                        middle = value.parse::<i32>()?;
                    }
                    for j in i+1..num_values {
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
                    total += middle;
                }
            }
        }
    }
    println!("The sum of the middle values of the correct updates is: {}", total);
    Ok(())
}

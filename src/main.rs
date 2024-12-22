mod aoc;
mod days;

use crate::aoc::Solution;
use crate::days::*;
use std::env;
use std::error::Error;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let maybe_solution_num = parse_args()?;
    let solutions = solutions();
    match maybe_solution_num {
        None => {
            for i in 0..solutions.len() {
                if i > 0 {
                    println!("");
                }
                println!("~- DAY {:0>2} -~", i + 1);
                let duration = run_solution(&solutions[i])?;
                println!("Run time: {}ms", duration.as_millis());
            }
        }
        Some(solution_num) => {
            if solution_num < 1 || solution_num > solutions.len() {
                return Err("argument out of range".into());
            }
            println!("~- DAY {:0>2} -~", solution_num);
            let duration = run_solution(&solutions[solution_num - 1])?;
            println!("Run time: {}ms", duration.as_millis());
        }
    }
    Ok(())
}

fn parse_args() -> Result<Option<usize>, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        return Err("too many arguments".into());
    }

    if args.len() == 0 {
        return Ok(None);
    }
    Ok(Some(args[0].parse::<usize>()?))
}

fn run_solution(solution: &Box<dyn Solution>) -> Result<Duration, Box<dyn Error>> {
    let timer = Instant::now();
    let (maybe_part1, maybe_part2) = solution.solve()?;
    let elapsed = timer.elapsed();
    if let Some(part1) = maybe_part1 {
        println!("Part 1: {part1}");
    }
    if let Some(part2) = maybe_part2 {
        println!("Part 2: {part2}");
    }
    Ok(elapsed)
}

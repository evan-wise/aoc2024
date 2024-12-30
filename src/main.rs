mod aoc;
mod days;

use crate::aoc::SolutionWrapper;
use crate::days::*;
use std::env;
use std::error::Error;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let maybe_solution_num = parse_args()?;
    let solutions = solutions();
    match maybe_solution_num {
        None => {
            let mut total = Duration::new(0, 0);
            for i in 0..solutions.len() {
                println!("~- DAY {:0>2} -~", i + 1);
                let duration = run_solution(&solutions[i])?;
                total += duration;
                println!("Run time: {}ms", duration.as_millis());
                println!("");
            }
            println!("Total solution time: {}ms", total.as_millis());
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

fn run_solution(solution: &Box<dyn SolutionWrapper>) -> Result<Duration, Box<dyn Error>> {
    let timer = Instant::now();
    let answers = solution.solve_string()?;
    let elapsed = timer.elapsed();
    println!("{answers}");
    Ok(elapsed)
}

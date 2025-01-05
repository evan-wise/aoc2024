mod aoc;
mod days;

use crate::aoc::SolutionWrapper;
use crate::days::*;
use std::env;
use std::error::Error;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let maybe_solution_num = parse_args()?;
    let mut solutions = solutions();
    match maybe_solution_num {
        None => {
            let mut total = Duration::new(0, 0);
            for i in 0..solutions.len() {
                println!("~- DAY {:0>2} -~", i + 1);
                let (parse_time, solution_time) = run_solution(&mut solutions[i])?;
                total += parse_time;
                total += solution_time;
                println!(
                    "Parse: {:.3}ms, Solve: {:.3}ms",
                    parse_time.as_micros() as f64 / 1000.0,
                    solution_time.as_micros() as f64 / 1000.0
                );
                println!("");
            }
            println!("Total: {}ms", total.as_millis());
        }
        Some(solution_num) => {
            if solution_num < 1 || solution_num > solutions.len() {
                return Err("argument out of range".into());
            }
            println!("~- DAY {:0>2} -~", solution_num);
            let (parse_time, solution_time) = run_solution(&mut solutions[solution_num - 1])?;
            println!(
                "Parse: {:.3}ms, Solve: {:.3}ms",
                parse_time.as_micros() as f64 / 1000.0,
                solution_time.as_micros() as f64 / 1000.0
            );
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

fn run_solution(
    solution: &mut Box<dyn SolutionWrapper>,
) -> Result<(Duration, Duration), Box<dyn Error>> {
    let parse_timer = Instant::now();
    solution.parse_input_wrapper()?;
    let parse_time = parse_timer.elapsed();
    let solution_timer = Instant::now();
    let answers = solution.solve_string()?;
    let solution_time = solution_timer.elapsed();
    println!("{answers}");
    Ok((parse_time, solution_time))
}

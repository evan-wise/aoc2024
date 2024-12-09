mod aoc;
mod days;

use crate::days::*;
use std::error::Error;
use std::time::{Duration, Instant};

fn run_solution(solution: &Box<dyn Solution>) -> Result<Duration, Box<dyn Error>> {
    let timer = Instant::now();
    solution.solve()?;
    Ok(timer.elapsed())
}

fn main() -> Result<(), Box<dyn Error>> {
    let solutions = solutions();
    for i in 0..solutions.len() {
        if i > 0 {
            println!("");
        }
        println!("~- DAY {:0>2} -~", i + 1);
        let duration = run_solution(&solutions[i])?;
        println!("Run time: {}ms", duration.as_millis());
    }
    Ok(())
}

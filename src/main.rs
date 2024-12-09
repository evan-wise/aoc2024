mod aoc;
mod days;

use std::error::Error;
use crate::days::{*};

fn main() -> Result<(), Box<dyn Error>> {
    println!("~- DAY 01 -~");
    day01::solve()?;
    println!("");
    println!("~- DAY 02 -~");
    day02::solve()?;
    println!("");
    println!("~- DAY 03 -~");
    day03::solve()?;
    println!("");
    println!("~- DAY 04 -~");
    day04::solve()?;
    println!("");
    println!("~- DAY 05 -~");
    day05::solve()?;
    println!("");
    println!("~- DAY 06 -~");
    day06::solve()?;
    println!("");
    println!("~- DAY 07 -~");
    day07::solve()?;
    Ok(())
}

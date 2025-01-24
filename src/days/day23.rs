use crate::aoc::{Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day23;

impl Day23 {
    pub fn new() -> Day23 {
        Day23 {}
    }
}

impl Solution for Day23 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        Ok(Answers::None)
    }
}

use crate::aoc::{Solution, Answers};
use std::error::Error;

#[derive(Debug)]
pub struct Day21;

impl Day21 {
    pub fn new() -> Day21 {
        Day21 { }
    }
}

impl Solution for Day21 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        Ok(Answers::None)
    }
}

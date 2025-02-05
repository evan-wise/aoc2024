use crate::aoc::{Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day25;

impl Day25 {
    pub fn new() -> Day25 {
        Day25 {}
    }
}

impl Solution for Day25 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        Ok(Answers::None)
    }
}

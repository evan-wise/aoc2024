use crate::aoc::{Solution, Answers};
use std::error::Error;

pub struct Day20;

impl Solution for Day20 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        Ok(Answers::from::<String, String>(None, None))
    }
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 { }
    }
}

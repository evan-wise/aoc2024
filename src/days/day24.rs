use crate::aoc::{Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day24;

impl Day24 {
    pub fn new() -> Day24 {
        Day24 {}
    }
}

impl Solution for Day24 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        Ok(Answers::None)
    }
}

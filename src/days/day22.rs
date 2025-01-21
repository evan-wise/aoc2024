use crate::aoc::{Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day22;

impl Day22 {
    pub fn new() -> Day22 {
        Day22 {}
    }
}

impl Solution for Day22 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        Ok(Answers::None)
    }
}

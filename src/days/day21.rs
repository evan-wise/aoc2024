use crate::aoc::{read_lines, Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day21 {
    codes: Vec<String>,
}

impl Day21 {
    pub fn new() -> Day21 {
        Day21 { codes: Vec::new() }
    }
}

impl Solution for Day21 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./examples/day21.txt";
        let lines = read_lines(filename)?;
        for  line in lines.flatten() {
            self.codes.push(line);
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        println!("{:?}", self.codes);
        Ok(Answers::None)
    }
}


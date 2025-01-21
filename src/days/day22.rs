use crate::aoc::{read_lines, Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day22 {
    seeds: Vec<usize>,
}

impl Day22 {
    pub fn new() -> Day22 {
        Day22 { seeds: Vec::new() }
    }
}

impl Solution for Day22 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day22.txt";
        let lines = read_lines(filename)?;
        self.seeds = lines.flatten().map(|l| l.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let part1: usize = self.seeds.iter().map(|s| {
            let mut result = *s;
            for _ in 0..2000 {
                result = prng(result);
            }
            result
        }).sum();
        Ok(Answers::part1(part1))
    }
}

const CUTOFF: usize = 2usize.pow(24);

fn prng(val: usize) -> usize {
    let mut result = val;
    result ^= result * 64;
    result %= CUTOFF;
    result ^= result / 32;
    result %= CUTOFF;
    result ^= result * 2048;
    result %= CUTOFF;
    result
}

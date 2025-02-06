use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

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
        self.seeds = lines
            .flatten()
            .map(|l| l.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let part1: usize = self
            .seeds
            .iter()
            .map(|&s| Prng::new(s).nth(1999))
            .flatten()
            .sum();
        let part2 = *self
            .seeds
            .iter()
            .fold(FxHashMap::default(), |mut a, &s| {
                let mut seen = FxHashSet::default();
                for (val, tup) in std::iter::once(s)
                    .chain(Prng::new(s).take(2000))
                    .map(|a| (a % 10) as isize)
                    .tuple_windows()
                    .map(|(a, b)| (b, b - a))
                    .tuple_windows()
                    .map(|((_, a), (_, b), (_, c), (v, d))| (v, (a, b, c, d)))
                {
                    if seen.insert(tup) {
                        *a.entry(tup).or_insert(0) += val;
                    }
                }
                a
            })
            .values()
            .max()
            .ok_or("no payoffs found")?;
        Ok(Answers::both(part1, part2))
    }
}

struct Prng {
    curr: usize,
}

impl Prng {
    fn new(seed: usize) -> Prng {
        Prng { curr: seed }
    }
}

impl Iterator for Prng {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr ^= self.curr << 6;
        self.curr &= 0x00FFFFFF;
        self.curr ^= self.curr >> 5;
        self.curr &= 0x00FFFFFF;
        self.curr ^= self.curr << 11;
        self.curr &= 0x00FFFFFF;
        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day22::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(12979353889usize, 1449isize));
        Ok(())
    }
}

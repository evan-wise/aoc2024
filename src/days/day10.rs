use crate::aoc::{read_chars, Answers, Solution};
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
pub struct Day10 {
    trailheads: Vec<(i32, i32)>,
    grid: Vec<Vec<u32>>,
    width: i32,
    height: i32,
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {
            trailheads: Vec::new(),
            grid: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn get_neighbors(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        let (i, j) = pos;
        if j - 1 >= 0 {
            neighbors.push((i, j - 1));
        }
        if j + 1 < self.height {
            neighbors.push((i, j + 1));
        }
        if i - 1 >= 0 {
            neighbors.push((i - 1, j));
        }
        if i + 1 < self.width {
            neighbors.push((i + 1, j));
        }
        neighbors
    }

    fn find_summits(&self, trailhead: (i32, i32), summits: &mut HashSet<(i32, i32)>) -> () {
        let (i, j) = trailhead;
        let neighbors = self.get_neighbors(trailhead);
        let cur_topo_height = self.grid[j as usize][i as usize];
        if cur_topo_height == 9 {
            summits.insert((i, j));
            return;
        }
        for (i, j) in neighbors {
            let next_topo_height = self.grid[j as usize][i as usize];
            if next_topo_height.saturating_sub(cur_topo_height) == 1 {
                self.find_summits((i, j), summits);
            }
        }
    }

    fn compute_rating(&self, trailhead: (i32, i32)) -> i32 {
        let mut rating = 0;
        let (i, j) = trailhead;
        let neighbors = self.get_neighbors(trailhead);
        let cur_topo_height = self.grid[j as usize][i as usize];
        if cur_topo_height == 9 {
            return 1;
        }
        for (i, j) in neighbors {
            let next_topo_height = self.grid[j as usize][i as usize];
            if next_topo_height.saturating_sub(cur_topo_height) == 1 {
                rating += self.compute_rating((i, j));
            }
        }
        rating
    }
}

impl Solution for Day10 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let chars = read_chars("./data/day10.txt")?;
        let mut i = 0;
        let mut j = 0;
        let mut row = Vec::new();
        for c in chars.flatten() {
            match c {
                '0'..='9' => {
                    row.push(c.to_string().parse::<u32>().unwrap());
                    if c == '0' {
                        self.trailheads.push((i, j));
                    }
                    i += 1;
                }
                '\n' => {
                    self.grid.push(row);
                    row = Vec::new();
                    i = 0;
                    j += 1;
                }
                _ => {
                    return Err(format!("invalid character {}", c).into());
                }
            }
        }
        self.height = self.grid.len() as i32;
        self.width = if self.height > 0 {
            self.grid[0].len()
        } else {
            0
        } as i32;
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut score = 0;
        for trailhead in &self.trailheads {
            let mut summits = HashSet::new();
            self.find_summits(*trailhead, &mut summits);
            score += summits.len();
        }

        let mut rating = 0;
        for trailhead in &self.trailheads {
            rating += self.compute_rating(*trailhead);
        }

        Ok(Answers::both(score, rating))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day10::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(629usize, 1242));
        Ok(())
    }
}

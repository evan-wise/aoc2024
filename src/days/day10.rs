use crate::aoc::read_chars;
use crate::days::Solution;
use std::collections::HashSet;

pub struct Day10;

impl Solution for Day10 {
    fn solve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let chars = read_chars("./data/day10.txt")?;
        let mut i = 0;
        let mut j = 0;
        let mut grid = Vec::new();
        let mut row = Vec::new();
        let mut trailheads = Vec::new();
        for c in chars.flatten() {
            match c {
                '0'..='9' => {
                    row.push(c.to_string().parse::<u32>().unwrap());
                    if c == '0' {
                        trailheads.push((i, j));
                    }
                    i += 1;
                },
                '\n' => {
                    grid.push(row);
                    row = Vec::new();
                    i = 0;
                    j += 1;
                },
                _ => {
                    return Err(format!("invalid character {}", c).into());
                },
            }
        }
        let map = Map::new(grid);
        let mut score = 0;
        for trailhead in trailheads {
            let mut summits = HashSet::new();
            map.find_summits(trailhead, &mut summits);
            score += summits.len();
        }
        println!("The total score for all the trailheads is: {}", score);
        Ok(())
    }
}

struct Map {
    width: i32,
    height: i32,
    grid: Vec<Vec<u32>>,
}

impl Map {
    fn new(grid: Vec<Vec<u32>>) -> Map {
        let height = grid.len() as i32;
        let width = if height > 0 { grid[0].len() } else { 0 } as i32;
        Map { width, height, grid }
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
}

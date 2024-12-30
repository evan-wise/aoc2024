use crate::aoc::{read_lines, Answers, Solution};
use std::collections::HashSet;
use std::error::Error;

pub struct Day12;

impl Solution for Day12 {
    type Part1 = i32;
    type Part2 = i32;

    fn solve(&self) -> Result<Answers<Self::Part1, Self::Part2>, Box<dyn Error>> {
        let lines = read_lines("./data/day12.txt")?;
        let grid = lines
            .flatten()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let map = Map::new(grid);
        let mut regions = Vec::new();
        let mut already_found: HashSet<(i32, i32)> = HashSet::new();
        for y in 0..map.height {
            for x in 0..map.width {
                let pos = (x, y);
                if !already_found.contains(&pos) {
                    let mut region = Region::new(map.grid[y as usize][x as usize]);
                    map.find_region(pos, &mut region);
                    already_found.extend(&region.plots);
                    regions.push(region);
                }
            }
        }
        let mut total_cost = 0;
        for region in &regions {
            total_cost += (region.plots.len() as i32) * region.perimeter;
        }
        let mut discount_cost = 0;
        for region in &regions {
            discount_cost += (region.plots.len() as i32) * region.count_sides();
        }
        Answers::ok(Some(total_cost), Some(discount_cost))
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(grid: Vec<Vec<char>>) -> Map {
        let height = grid.len() as i32;
        let mut width = 0;
        if height > 0 {
            width = grid[0].len() as i32;
        }
        Map {
            grid,
            height,
            width,
        }
    }

    // Up, Down, Left, Right
    fn neighbors(&self, pos: (i32, i32)) -> Vec<Option<(i32, i32)>> {
        let mut neighbors = Vec::new();
        let (x, y) = pos;
        if y > 0 {
            neighbors.push(Some((x, y - 1)));
        } else {
            neighbors.push(None);
        }
        if y < self.height - 1 {
            neighbors.push(Some((x, y + 1)));
        } else {
            neighbors.push(None);
        }
        if x > 0 {
            neighbors.push(Some((x - 1, y)));
        } else {
            neighbors.push(None);
        }
        if x < self.width - 1 {
            neighbors.push(Some((x + 1, y)));
        } else {
            neighbors.push(None);
        }
        neighbors
    }

    fn find_region(&self, pos: (i32, i32), region: &mut Region) {
        let (x, y) = pos;
        let plant = self.grid[y as usize][x as usize];
        if plant != region.plant {
            region.perimeter += 1;
            return;
        }
        region.plots.insert(pos);

        for neighbor in self.neighbors(pos) {
            if let Some(neighbor_pos) = neighbor {
                if !region.plots.contains(&neighbor_pos) {
                    self.find_region(neighbor_pos, region);
                }
            } else {
                region.perimeter += 1;
            }
        }
    }
}

#[derive(Debug)]
struct Region {
    plant: char,
    plots: HashSet<(i32, i32)>,
    perimeter: i32,
}

impl Region {
    fn new(plant: char) -> Region {
        Region {
            plant,
            plots: HashSet::new(),
            perimeter: 0,
        }
    }

    fn count_sides(&self) -> i32 {
        let mut already_seen = HashSet::new();
        let mut count = 0;
        for &pos in &self.plots {
            let (x, y) = pos;
            if already_seen.contains(&pos) {
                continue;
            }
            // Find top sides
            if !self.plots.contains(&(x, y - 1)) {
                count += 1;
                already_seen.insert(pos);
                let mut i = 1;
                while self.plots.contains(&(x + i, y)) && !self.plots.contains(&(x + i, y - 1)) {
                    already_seen.insert((x + i, y));
                    i += 1;
                }
                i = 1;
                while self.plots.contains(&(x - i, y)) && !self.plots.contains(&(x - i, y - 1)) {
                    already_seen.insert((x - i, y));
                    i += 1;
                }
            }
        }
        already_seen = HashSet::new();
        for &pos in &self.plots {
            let (x, y) = pos;
            if already_seen.contains(&pos) {
                continue;
            }
            // Find bottom sides
            if !self.plots.contains(&(x, y + 1)) {
                count += 1;
                already_seen.insert(pos);
                let mut i = 1;
                while self.plots.contains(&(x + i, y)) && !self.plots.contains(&(x + i, y + 1)) {
                    already_seen.insert((x + i, y));
                    i += 1;
                }
                i = 1;
                while self.plots.contains(&(x - i, y)) && !self.plots.contains(&(x - i, y + 1)) {
                    already_seen.insert((x - i, y));
                    i += 1;
                }
            }
        }
        already_seen = HashSet::new();
        for &pos in &self.plots {
            let (x, y) = pos;
            if already_seen.contains(&pos) {
                continue;
            }
            // Find left sides
            if !self.plots.contains(&(x - 1, y)) {
                count += 1;
                already_seen.insert(pos);
                let mut i = 1;
                while self.plots.contains(&(x, y + i)) && !self.plots.contains(&(x - 1, y + i)) {
                    already_seen.insert((x, y + i));
                    i += 1;
                }
                i = 1;
                while self.plots.contains(&(x, y - i)) && !self.plots.contains(&(x - 1, y - i)) {
                    already_seen.insert((x, y - i));
                    i += 1;
                }
            }
        }
        already_seen = HashSet::new();
        for &pos in &self.plots {
            let (x, y) = pos;
            if already_seen.contains(&pos) {
                continue;
            }
            // Find right sides
            if !self.plots.contains(&(x + 1, y)) {
                count += 1;
                already_seen.insert(pos);
                let mut i = 1;
                while self.plots.contains(&(x, y + i)) && !self.plots.contains(&(x + 1, y + i)) {
                    already_seen.insert((x, y + i));
                    i += 1;
                }
                i = 1;
                while self.plots.contains(&(x, y - i)) && !self.plots.contains(&(x + 1, y - i)) {
                    already_seen.insert((x, y - i));
                    i += 1;
                }
            }
        }
        count
    }
}

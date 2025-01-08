use crate::aoc::{read_lines, Answers, Direction, Map, Position, Solution};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day18 {
    bytes: Vec<Position>,
    num_bytes: usize,
    size: usize,
    corrupted: FxHashSet<Position>,
}

impl Day18 {
    pub fn new() -> Day18 {
        Day18 {
            bytes: Vec::new(),
            num_bytes: 0,
            size: 0,
            corrupted: FxHashSet::default(),
        }
    }

    fn minimal_path(&self) -> Option<usize> {
        let start = (0, 0);
        let end = (self.size - 1, self.size - 1);
        let mut heap = BinaryHeap::from([(Reverse(0), start)]);
        let mut visited = FxHashSet::default();
        let mut low_dists = FxHashMap::default();
        while let Some((Reverse(dist), pos)) = heap.pop() {
            let prev_dist = *low_dists.get(&pos).unwrap_or(&usize::MAX);
            if dist >= prev_dist {
                continue;
            }
            low_dists.insert(pos, dist);

            if !visited.insert(pos) {
                continue;
            }

            if pos == end {
                continue;
            }

            for d in Direction::all() {
                if let Some(((x, y), Cell::Safe)) = d.go(self, pos) {
                    heap.push((Reverse(dist + 1), (x, y)));
                }
            }
        }

        if low_dists.contains_key(&end) {
            Some(low_dists[&end])
        } else {
            None
        }
    }
}

impl Solution for Day18 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day18.txt";
        let lines = read_lines(filename)?;
        self.num_bytes = if filename.contains("/data/") {
            1024
        } else if filename.contains("/examples/") {
            12
        } else {
            return Err("expected path to contain \"/data/\" or \"/examples/\"".into());
        };
        self.size = if self.num_bytes == 1024 { 71 } else { 7 };
        for (line_num, line) in lines.flatten().enumerate() {
            let parts = line
                .split(",")
                .map(|p| p.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;
            if parts.len() != 2 {
                return Err(format!("invalid line {line_num}").into());
            }
            self.bytes.push((parts[0], parts[1]));
        }
        if self.bytes.len() < self.num_bytes {
            return Err(format!("byte stream has less than {} bytes", self.num_bytes).into());
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        for i in 0..self.num_bytes {
            self.corrupted.insert(self.bytes[i]);
        }
        let dist = self.minimal_path();
        let mut byte_str = String::new();
        for i in self.num_bytes..self.bytes.len() {
            let byte = self.bytes[i];
            self.corrupted.insert(byte);
            if let None = self.minimal_path() {
                byte_str = format!("{},{}", byte.0, byte.1);
                break;
            }
        }
        Ok(Answers::from(dist, Some(byte_str)))
    }
}

impl Map for Day18 {
    type Cell = Cell;

    fn width(&self) -> usize {
        self.size
    }

    fn height(&self) -> usize {
        self.size
    }

    fn get(&self, pos: Position) -> Option<Cell> {
        let (x, y) = pos;
        if x >= self.size || y >= self.size {
            return None;
        }
        if self.corrupted.contains(&pos) {
            Some(Cell::Corrupted)
        } else {
            Some(Cell::Safe)
        }
    }
}

#[derive(Debug)]
pub enum Cell {
    Safe,
    Corrupted,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Safe => write!(f, "."),
            Self::Corrupted => write!(f, "#"),
        }
    }
}

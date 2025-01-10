use crate::aoc::{read_lines, Answers, Direction, Map, Position, Solution};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day20 {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 {
            grid: Vec::new(),
            width: 0,
            height: 0,
            start: (0, 0),
            end: (0, 0),
        }
    }
}

impl Solution for Day20 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./examples/day20.txt")?;
        for (y, line) in lines.flatten().enumerate() {
            self.grid.push(
                line.chars()
                    .map(|c| Cell::from(c))
                    .collect::<Result<_, _>>()?,
            );
            if let Some(x) = line.chars().position(|c| c == 'S') {
                self.start = (x, y);
            }
            if let Some(x) = line.chars().position(|c| c == 'E') {
                self.end = (x, y);
            }
        }
        self.height = self.grid.len();
        self.width = if self.height > 0 {
            self.grid[0].len()
        } else {
            0
        };
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        if let Some(time) = self.minimal_path(self.start, self.end) {
            println!("{time}");
        }
        Ok(Answers::from::<String, String>(None, None))
    }
}

impl Map for Day20 {
    type Cell = Cell;
    fn get(&self, pos: Position) -> Option<Self::Cell> {
        let (x, y) = pos;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.grid[y][x])
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Day20 {
    fn minimal_path(&self, start: Position, end: Position) -> Option<usize> {
        let mut heap = BinaryHeap::from([(Reverse(0), start)]);
        let mut low_dists = FxHashMap::default();
        while let Some((Reverse(dist), pos)) = heap.pop() {
            let prev_dist = *low_dists.get(&pos).unwrap_or(&usize::MAX);
            if dist >= prev_dist {
                continue;
            }
            if let Some(_) = low_dists.insert(pos, dist) {
                continue;
            }
            if pos == end {
                continue;
            }
            for d in Direction::all() {
                if let Some(((x, y), Cell::Empty)) = d.go(self, pos) {
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

#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Empty,
    Wall,
}

impl Cell {
    pub fn from(c: char) -> Result<Self, String> {
        match c {
            '#' => Ok(Self::Wall),
            '.' | 'S' | 'E' => Ok(Self::Empty),
            _ => Err("invalid character".into()),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Wall => write!(f, "#"),
        }
    }
}

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
        if let (Some(base), _, backtracks) =
            self.backtrack_minimal_path(Cell::Empty, self.start, self.end)
        {
            let visited = backtrack(self.end, &backtracks);
            Ok(Answers::from::<_, _>(Some(base), Some(visited.len())))
        } else {
            Ok(Answers::from::<String, String>(None, None))
        }
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

fn backtrack(
    from: Position,
    backtracks: &FxHashMap<Position, FxHashSet<Position>>,
) -> FxHashSet<Position> {
    let mut visited = FxHashSet::default();
    let mut stack = vec![from];
    let empty = FxHashSet::default();
    while let Some(pos) = stack.pop() {
        if visited.insert(pos) {
            stack.extend(backtracks.get(&pos).unwrap_or(&empty));
        }
    }
    visited
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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

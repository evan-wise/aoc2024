use crate::aoc::{read_lines, Answers, Direction, Map, Position, Solution};
use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day20 {
    live: bool,
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 {
            live: false,
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
        let filename = "./data/day20.txt";
        let lines = read_lines(filename)?;
        self.live = if filename.contains("/data/") { true } else { false };
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
        if let (Some(base), lows, backtracks) =
            self.backtrack_minimal_path(Cell::Empty, self.start, self.end)
        {
            let (_, reverse_lows) = self.minimal_path(Cell::Empty, self.end, self.start);
            let mut cheats = FxHashMap::default();
            let mut good_cheats = 0 as usize;
            let cheat_threshold = if self.live { 100 } else { 50 };
            let visited = backtrack(self.end, &backtracks);
            for pos in &visited {
                for d in Direction::all() {
                    if let Some((p1, Cell::Wall)) = self.go(d, *pos) {
                        if let Some((p2, Cell::Empty)) = self.go(d, p1) {
                            if cheats.contains_key(&(p1, p2)) {
                                continue;
                            }
                            if let (Some(l), Some(r)) = (lows.get(&pos), reverse_lows.get(&p2)) {
                                let time = r + 2 + l;
                                if time < base {
                                    cheats.insert((p1, p2), time);
                                    if base - time >= cheat_threshold {
                                        good_cheats += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(Answers::part1(good_cheats))
        } else {
            Ok(Answers::none())
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

#[allow(dead_code)]
impl Day20 {
    fn print_map(&self, visited: &FxHashSet<Position>) {
        for (y, row) in self.grid.iter().enumerate() {
            println!(
                "{}",
                row.iter()
                    .enumerate()
                    .map(|(x, c)| if (x, y) == self.end {
                        "E".to_string()
                    } else if (x, y) == self.start {
                        "S".to_string()
                    } else if visited.contains(&(x, y)) {
                        "O".to_string()
                    } else {
                        format!("{c}")
                    })
                    .collect::<String>()
            );
        }
    }
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

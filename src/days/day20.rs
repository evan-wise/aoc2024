use crate::aoc::grid::Grid;
use crate::aoc::{read_lines, Answers, Direction, Map, Position, Solution};
use rustc_hash::{FxHashMap, FxHashSet};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day20 {
    live: bool,
    grid: Grid<Cell>,
    start: Position,
    end: Position,
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 {
            live: false,
            grid: Grid::new(),
            start: (0, 0),
            end: (0, 0),
        }
    }
}

impl Solution for Day20 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day20.txt";
        let lines = read_lines(filename)?;
        self.live = if filename.contains("/data/") {
            true
        } else {
            false
        };
        self.grid.width = if self.live { 141 } else { 15 };
        self.grid.height = self.grid.width;
        for (y, line) in lines.flatten().enumerate() {
            self.grid.extend(
                line.chars()
                    .map(|c| Cell::from(c))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            if let Some(x) = line.chars().position(|c| c == 'S') {
                self.start = (x, y);
            }
            if let Some(x) = line.chars().position(|c| c == 'E') {
                self.end = (x, y);
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let (maybe_base, lows, backtracks) =
            self.backtrack_minimal_path(Cell::Empty, self.start, self.end);
        let base = maybe_base.ok_or("no solution to input maze")?;
        let (_, reverse_lows) = self.minimal_path(Cell::Empty, self.end, self.start);
        let mut cheats = FxHashMap::default();
        for pos in &backtrack(self.end, &backtracks) {
            self.explore_two_steps(*pos, base, &lows, &reverse_lows, &mut cheats);
        }
        let cheat_threshold = if self.live { 100 } else { 50 };
        let good_cheats = cheats.iter().fold(0usize, |a, (_, time)| {
            a + if base - time >= cheat_threshold { 1 } else { 0 }
        });
        Ok(Answers::part1(good_cheats))
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

impl Day20 {
    fn explore_two_steps(
        &self,
        from: Position,
        base: usize,
        lows: &FxHashMap<Position, usize>,
        reverse_lows: &FxHashMap<Position, usize>,
        cheats: &mut FxHashMap<(Position, Position), usize>,
    ) {
        for d in Direction::all() {
            if let Some((p1, Cell::Wall)) = self.go(d, from) {
                if let Some((p2, Cell::Empty)) = self.go(d, p1) {
                    if cheats.contains_key(&(p1, p2)) {
                        return;
                    }
                    if let (Some(l), Some(r)) = (lows.get(&from), reverse_lows.get(&p2)) {
                        let time = r + 2 + l;
                        if time < base {
                            cheats.insert((p1, p2), time);
                        }
                    }
                }
            }
        }
    }
}

impl Map for Day20 {
    type Cell = Cell;
    fn get(&self, pos: Position) -> Option<Self::Cell> {
        self.grid.get(&pos).copied()
    }

    fn width(&self) -> usize {
        self.grid.width
    }

    fn height(&self) -> usize {
        self.grid.height
    }
}

#[allow(dead_code)]
impl Day20 {
    fn print_map(&self, visited: &FxHashSet<Position>) {
        for (pos, cell) in self.grid.iter() {
            print!(
                "{}",
                if pos == self.end {
                    "E".to_string()
                } else if pos == self.start {
                    "S".to_string()
                } else if visited.contains(&pos) {
                    "O".to_string()
                } else {
                    format!("{cell}")
                }
            );
            if pos.0 == self.grid.width - 1 {
                print!("\n");
            }
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

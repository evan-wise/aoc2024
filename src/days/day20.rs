use crate::aoc::grid::Grid;
use crate::aoc::{read_lines, Answers, Direction, Map, Position, Solution};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
        let visited = backtrack(self.end, &backtracks);
        for pos in &visited {
            self.explore(2, *pos, base, &lows, &reverse_lows, &mut cheats);
        }
        let thresh = if self.live { 100 } else { 50 };
        let part1 = count_good_cheats(&cheats, base, thresh);
        for pos in &visited {
            self.explore(20, *pos, base, &lows, &reverse_lows, &mut cheats);
        }
        let part2 = count_good_cheats(&cheats, base, thresh);
        Ok(Answers::both(part1, part2))
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

fn count_good_cheats(
    cheats: &FxHashMap<(Position, Position), usize>,
    base: usize,
    thresh: usize,
) -> usize {
    cheats.iter().fold(0usize, |a, (_, time)| {
        a + if base - time >= thresh { 1 } else { 0 }
    })
}

impl Day20 {
    fn explore(
        &self,
        max_steps: usize,
        from: Position,
        base: usize,
        forward_lows: &FxHashMap<Position, usize>,
        reverse_lows: &FxHashMap<Position, usize>,
        cheats: &mut FxHashMap<(Position, Position), usize>,
    ) {
        let mut heap = BinaryHeap::from([(Reverse(0), from, self.get(&from).unwrap())]);
        let mut lows = FxHashMap::default();
        while let Some((Reverse(dist), pos, cell)) = heap.pop() {
            let prev_dist = *lows.get(&pos).unwrap_or(&usize::MAX);
            if dist > prev_dist {
                continue;
            }
            if prev_dist < usize::MAX {
                continue;
            }
            lows.insert(pos, dist);
            if let (Cell::Empty, Some(l), Some(r)) =
                (cell, forward_lows.get(&from), reverse_lows.get(&pos))
            {
                let time = r + dist + l;
                let prev_cheat = *cheats.get(&(from, pos)).unwrap_or(&usize::MAX);
                if time < base && time < prev_cheat {
                    cheats.insert((from, pos), time);
                }
            }
            if dist >= max_steps {
                continue;
            }
            for d in Direction::all() {
                if let Some(((x, y), c)) = self.go(d, &pos) {
                    heap.push((Reverse(dist + 1), (x, y), c));
                }
            }
        }
    }
}

impl Map for Day20 {
    type Cell = Cell;
    fn get(&self, pos: &Position) -> Option<&Self::Cell> {
        self.grid.get(pos)
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

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
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

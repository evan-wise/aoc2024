use crate::aoc::{read_chars, Answers, Direction, Map, Position, Solution};
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day06 {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    guard: Guard,
    history: Vec<Guard>,
    history_lookup: HashSet<Guard>,
    visited: HashSet<Position>,
    loops: HashSet<Position>,
    exits: HashSet<Position>,
}

impl Solution for Day06 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let mut row = Vec::new();
        let mut y = 0;
        let mut x = 0;
        let chars = read_chars("./data/day06.txt")?;
        for char in chars.flatten() {
            match char {
                '.' => {
                    row.push(Cell::Empty);
                    x += 1;
                }
                '#' => {
                    row.push(Cell::Obstacle);
                    x += 1;
                }
                '^' => {
                    row.push(Cell::Empty);
                    self.place_guard((x, y));
                    x += 1;
                }
                '\n' => {
                    self.grid.push(row);
                    row = Vec::new();
                    y += 1;
                    x = 0;
                }
                _ => {
                    return Err("invalid character".into());
                }
            }
        }
        self.height = self.grid.len();
        self.width = if self.height > 0 { self.grid[0].len() } else { 0 };
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        self.simulate();
        let part1 = self.visited.len();

        let history = self.history[0..self.history.len() - 1].to_vec();

        for guard in &history {
            if let Some((pos, _)) = guard.direction.go(self, guard.position) {
                let (x, y) = pos;

                if self.exits.contains(&pos) || self.loops.contains(&pos) || self.grid[y][x] == Cell::Obstacle {
                    continue;
                }

                self.reset();
                self.place_guard(history[0].position);

                self.grid[y][x] = Cell::Obstacle;
                if let SimulationResult::Loop = self.simulate() {
                    self.loops.insert(pos);
                } else {
                    self.exits.insert(pos);
                }
                self.grid[y][x] = Cell::Empty;
            }
        }
        let part2 = self.loops.len();

        Ok(Answers::from(Some(part1), Some(part2)))
    }
}

impl Map for Day06 {
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

impl Day06 {
    pub fn new() -> Day06 {
        Day06 {
            grid: Vec::new(),
            width: 0,
            height: 0,
            guard: Guard::new(),
            history: Vec::new(),
            history_lookup: HashSet::new(),
            visited: HashSet::new(),
            loops: HashSet::new(),
            exits: HashSet::new(),
        }
    }

    pub fn reset(&mut self) {
        self.history = Vec::new();
        self.history_lookup = HashSet::new();
        self.visited = HashSet::new();
        self.guard = Guard::new();
    }

    fn place_guard(&mut self, position: Position) -> &Guard {
        self.guard.position = position;
        self.visited.insert(position);
        self.history.push(self.guard);
        self.history_lookup.insert(self.guard);
        &self.guard
    }

    fn step(&mut self) -> Option<bool> {
        match self.guard.direction.go(self, self.guard.position) {
            Some((_, Cell::Obstacle)) => {
                self.guard.direction = self.guard.direction.right();
                self.history.push(self.guard);
                Some(!self.history_lookup.insert(self.guard))
            }
            Some((pos, Cell::Empty)) => {
                self.guard.position = pos;
                self.history.push(self.guard);
                self.visited.insert(self.guard.position);
                Some(!self.history_lookup.insert(self.guard))
            }
            None => None,
        }
    }

    fn simulate(&mut self) -> SimulationResult {
        while let Some(loop_detected) = self.step() {
            if loop_detected {
                return SimulationResult::Loop;
            }
        }
        SimulationResult::Exit
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Obstacle,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Obstacle => write!(f, "#"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn new() -> Guard {
        Guard {
            position: (0, 0),
            direction: Direction::Up,
        }
    }
}

#[derive(Debug)]
enum SimulationResult {
    Exit,
    Loop,
}

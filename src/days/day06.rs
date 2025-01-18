use crate::aoc::grid::Grid;
use crate::aoc::{read_chars, Answers, Direction, Map, Position, Solution};
use rustc_hash::FxHashSet;
use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug)]
pub struct Day06 {
    grid: Grid<Cell>,
    guard: Guard,
    history: Vec<Guard>,
    visited: FxHashSet<Position>,
    seen: FxHashSet<Guard>,
    loops: FxHashSet<Position>,
}

impl Solution for Day06 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day06.txt";
        let width = if filename.contains("/data/") { 130 } else { 10 };
        let height = width;
        self.grid = Grid::fill(Cell::Empty, width, height);
        let mut y = 0;
        let mut x = 0;
        let chars = read_chars(filename)?;
        for char in chars.flatten() {
            match char {
                '.' => {
                    x += 1;
                }
                '#' => {
                    self.grid[(x, y)] = Cell::Obstacle;
                    x += 1;
                }
                '^' => {
                    self.place_guard((x, y));
                    x += 1;
                }
                '\n' => {
                    y += 1;
                    x = 0;
                }
                _ => {
                    return Err("invalid character".into());
                }
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        self.simulate(SimulationType::History);
        let part1 = self.visited.len();
        let history = self.history[0..self.history.len() - 1].to_vec();

        for i in 0..history.len() {
            let guard = history[i];
            if let Some((pos, _)) = self.go(guard.direction, &guard.position) {
                if self.loops.contains(&pos) || self.grid[pos] == Cell::Obstacle {
                    continue;
                }

                self.place_guard(history[0].position);

                self.grid[pos] = Cell::Obstacle;
                if let SimulationResult::Loop = self.simulate(SimulationType::Test) {
                    self.loops.insert(pos);
                }
                self.grid[pos] = Cell::Empty;
            }
        }
        let part2 = self.loops.len();

        Ok(Answers::both(part1, part2))
    }
}

impl Map for Day06 {
    type Cell = Cell;

    fn get(&self, pos: &Position) -> Option<&Self::Cell> {
        self.grid.get(&pos)
    }

    fn width(&self) -> usize {
        self.grid.width
    }

    fn height(&self) -> usize {
        self.grid.height
    }
}

impl Day06 {
    pub fn new() -> Day06 {
        Day06 {
            grid: Grid::new(),
            guard: Guard::new(),
            history: Vec::new(),
            visited: FxHashSet::default(),
            seen: FxHashSet::default(),
            loops: FxHashSet::default(),
        }
    }

    fn place_guard(&mut self, position: Position) -> &Guard {
        self.guard = Guard::new();
        self.guard.position = position;
        &self.guard
    }

    fn step(&mut self) -> Option<(Guard, Cell)> {
        match self.go(self.guard.direction, &self.guard.position) {
            Some((_, Cell::Obstacle)) => {
                self.guard.direction = self.guard.direction.right();
                Some((self.guard, Cell::Obstacle))
            }
            Some((pos, Cell::Empty)) => {
                self.guard.position = pos;
                Some((self.guard, Cell::Empty))
            }
            None => None,
        }
    }

    fn simulate(&mut self, simulation_type: SimulationType) -> SimulationResult {
        self.seen.clear();
        self.seen.insert(self.guard);
        if let SimulationType::History = simulation_type {
            self.history.clear();
            self.history.push(self.guard);
            self.visited.clear();
            self.visited.insert(self.guard.position);
        }
        while let Some((guard, _)) = self.step() {
            if !self.seen.insert(guard) {
                return SimulationResult::Loop;
            }
            if let SimulationType::History = simulation_type {
                self.history.push(guard);
                self.visited.insert(guard.position);
            }
        }
        SimulationResult::Exit
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

impl Hash for Guard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (x, y) = self.position;
        let i = ((y as u32) << 15) + ((x as u32) << 2) + (self.direction as u32);
        i.hash(state)
    }
}

#[derive(Debug)]
enum SimulationType {
    History,
    Test,
}

#[derive(Debug)]
enum SimulationResult {
    Exit,
    Loop,
}

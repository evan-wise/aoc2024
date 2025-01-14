use crate::aoc::{read_chars, Answers, Direction, Map, Position, Solution};
use rustc_hash::FxHashSet;
use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;
use std::time::Instant;

#[derive(Debug)]
pub struct Day06 {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    guard: Guard,
    history: Vec<Guard>,
    visited: FxHashSet<Position>,
    seen: FxHashSet<Guard>,
    loops: FxHashSet<Position>,
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
        self.width = if self.height > 0 {
            self.grid[0].len()
        } else {
            0
        };
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        self.simulate(SimulationType::History);
        let part1 = self.visited.len();
        let history = self.history[0..self.history.len() - 1].to_vec();

        let mut times = vec![0; history.len()];
        for i in 0..history.len() {
            let timer = Instant::now();
            let guard = history[i];
            if let Some((pos, _)) = self.go(guard.direction, guard.position) {
                let (x, y) = pos;

                if self.loops.contains(&pos) || self.grid[y][x] == Cell::Obstacle {
                    continue;
                }

                self.place_guard(history[0].position);

                self.grid[y][x] = Cell::Obstacle;
                if let SimulationResult::Loop = self.simulate(SimulationType::Test) {
                    self.loops.insert(pos);
                }
                self.grid[y][x] = Cell::Empty;
            }
            times[i] = timer.elapsed().as_micros();
        }
        let n = times.len() as f64;
        let avg = times.iter().fold(0.0, |a, t| a + *t as f64 / n);
        println!("simulations: {n:.0}, average simulation: {avg:.0}μs");
        let part2 = self.loops.len();

        Ok(Answers::both(part1, part2))
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
        match self.go(self.guard.direction, self.guard.position) {
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

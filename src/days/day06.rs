use crate::aoc::{read_chars, Answers, Direction, Position, Solution};
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
pub struct Day06 {
    map: Vec<Vec<Cell>>,
    guard: Guard,
    history: Vec<Guard>,
    history_lookup: HashSet<Guard>,
    visited: HashSet<Position>,
    loops: HashSet<Position>,
}

impl Day06 {
    pub fn new() -> Day06 {
        Day06 {
            map: Vec::new(),
            guard: Guard::new(),
            history: Vec::new(),
            history_lookup: HashSet::new(),
            visited: HashSet::new(),
            loops: HashSet::new(),
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

    fn check_bounds(&self) -> bool {
        let (i, j) = self.guard.position;
        let height = self.map.len();
        if height == 0 {
            return false;
        }
        let width = self.map[0].len();
        if width == 0 {
            return false;
        }
        match self.guard.direction {
            Direction::Up => i != 0,
            Direction::Down => i < height - 1,
            Direction::Left => j != 0,
            Direction::Right => j < width - 1,
        }
    }

    fn step(&mut self) -> Option<bool> {
        if !self.check_bounds() {
            return None
        }
        let (i, j) = self.guard.position;
        let (k, l, turn) = match self.guard.direction {
            Direction::Up => (i - 1, j, Direction::Right),
            Direction::Down => (i + 1, j, Direction::Left),
            Direction::Left => (i, j - 1, Direction::Up),
            Direction::Right => (i, j + 1, Direction::Down),
        };
        match self.map[k][l] {
            Cell::Obstacle => {
                self.guard.direction = turn;
                self.history.push(self.guard);
                Some(!self.history_lookup.insert(self.guard))
            }
            Cell::Empty => {
                self.guard.position = (k, l);
                self.history.push(self.guard);
                self.visited.insert(self.guard.position);
                Some(!self.history_lookup.insert(self.guard))
            }
        }
    }

    fn simulate(&mut self) -> SimulationResult {
        loop {
            if let Some(loop_detected) = self.step() {
                if loop_detected {
                    return SimulationResult::Loop;
                }
            } else {
                break;
            }
        }
        SimulationResult::Exit
    }
}

impl Solution for Day06 {
    type Part1 = usize;
    type Part2 = usize;

    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let mut row = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let chars = read_chars("./data/day06.txt")?;
        for char in chars.flatten() {
            match char {
                '.' => {
                    row.push(Cell::Empty);
                    j += 1;
                }
                '#' => {
                    row.push(Cell::Obstacle);
                    j += 1;
                }
                '^' => {
                    row.push(Cell::Empty);
                    self.place_guard((i, j));
                    j += 1;
                }
                '\n' => {
                    self.map.push(row);
                    row = Vec::new();
                    i += 1;
                    j = 0;
                }
                _ => {
                    return Err("invalid character".into());
                }
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers<Self::Part1, Self::Part2>, Box<dyn Error>> {
        self.simulate();
        let part1 = self.visited.len();

        let history = self.history[0..self.history.len() - 1].to_vec();
        for guard in &history {
            let (i, j) = guard.position;
            let (k, l) = match guard.direction {
                Direction::Up => (i - 1, j),
                Direction::Down => (i + 1, j),
                Direction::Left => (i, j - 1),
                Direction::Right => (i, j + 1),
            };
            if self.loops.contains(&(k, l)) || self.map[k][l] == Cell::Obstacle {
                continue;
            }
            self.reset();
            self.place_guard(history[0].position);
            self.map[k][l] = Cell::Obstacle;
            if let SimulationResult::Loop = self.simulate() {
                self.loops.insert((k, l));
            }
            self.map[k][l] = Cell::Empty;
        }
        let part2 = self.loops.len();

        Answers::ok(Some(part1), Some(part2))
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cell {
    Empty,
    Obstacle,
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

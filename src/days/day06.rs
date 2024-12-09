use crate::aoc::{read_chars, FileCharIterator};
use crate::days::Solution;
use std::collections::HashSet;
use std::error::Error;

pub struct Day06;

impl Solution for Day06 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        let chars = read_chars("./data/map.txt")?;

        let mut state = parse_input(chars)?;

        do_part1(&mut state)?;

        do_part2(&state)?;

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Debug)]
enum CellState {
    Unvisited,
    Visited,
    Obstacle,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Guard {
    pub position: (usize, usize),
    pub direction: Direction,
}

impl Guard {
    pub fn new() -> Guard {
        Guard {
            position: (0, 0),
            direction: Direction::Up,
        }
    }
}

struct StepResult {
    cell: CellState,
    loop_detected: bool,
}

struct State {
    map: Vec<Vec<CellState>>,
    guard: Guard,
    history: Vec<Guard>,
    history_lookup: HashSet<Guard>,
}

impl State {
    pub fn new(map: Vec<Vec<CellState>>) -> State {
        State {
            map,
            guard: Guard::new(),
            history: Vec::new(),
            history_lookup: HashSet::new(),
        }
    }

    pub fn map(&self) -> &Vec<Vec<CellState>> {
        &self.map
    }

    pub fn history(&self) -> &Vec<Guard> {
        &self.history
    }

    pub fn place_guard(&mut self, position: (usize, usize)) -> &Guard {
        self.guard.position = position;
        self.map[position.0][position.1] = CellState::Visited;
        self.history.push(self.guard.clone());
        self.history_lookup.insert(self.guard.clone());
        &self.guard
    }

    pub fn check_bounds(&self) -> bool {
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

    pub fn do_step(&mut self) -> Result<StepResult, Box<dyn Error>> {
        if !self.check_bounds() {
            return Err("out of bounds".into());
        }
        let (i, j) = self.guard.position;
        let (k, l, dir) = match self.guard.direction {
            Direction::Up => (i - 1, j, Direction::Right),
            Direction::Down => (i + 1, j, Direction::Left),
            Direction::Left => (i, j - 1, Direction::Up),
            Direction::Right => (i, j + 1, Direction::Down),
        };
        match self.map[k][l] {
            CellState::Obstacle => {
                self.guard.direction = dir;
                self.history.push(self.guard.clone());
                self.history_lookup.insert(self.guard.clone());
                Ok(StepResult {
                    cell: CellState::Obstacle,
                    loop_detected: false,
                })
            }
            CellState::Unvisited => {
                self.guard.position = (k, l);
                self.map[k][l] = CellState::Visited;
                self.history.push(self.guard.clone());
                self.history_lookup.insert(self.guard.clone());
                Ok(StepResult {
                    cell: CellState::Unvisited,
                    loop_detected: false,
                })
            }
            CellState::Visited => {
                self.guard.position = (k, l);
                self.history.push(self.guard.clone());
                let loop_detected = !self.history_lookup.insert(self.guard.clone());
                Ok(StepResult {
                    cell: CellState::Visited,
                    loop_detected,
                })
            }
        }
    }

    pub fn fresh_map(&self) -> Vec<Vec<CellState>> {
        let mut new_map = Vec::new();
        for row in &self.map {
            let mut new_row = Vec::new();
            for cell in row {
                match cell {
                    CellState::Unvisited | CellState::Visited => {
                        new_row.push(CellState::Unvisited);
                    }
                    CellState::Obstacle => {
                        new_row.push(CellState::Obstacle);
                    }
                }
            }
            new_map.push(new_row);
        }
        new_map
    }
}

fn parse_input(chars: FileCharIterator) -> Result<State, Box<dyn Error>> {
    let mut map = Vec::new();
    let mut row = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let mut position = (0, 0);
    for char in chars.flatten() {
        match char {
            '.' => {
                row.push(CellState::Unvisited);
                j += 1;
            }
            '#' => {
                row.push(CellState::Obstacle);
                j += 1;
            }
            '^' => {
                row.push(CellState::Unvisited);
                position = (i, j);
                j += 1;
            }
            '\n' => {
                map.push(row);
                row = Vec::new();
                i += 1;
                j = 0;
            }
            _ => {
                return Err("invalid character".into());
            }
        }
    }
    let mut state = State::new(map);
    state.place_guard(position);
    Ok(state)
}

fn do_part1(state: &mut State) -> Result<(), Box<dyn Error>> {
    let mut num_positions = 1;
    loop {
        match state.do_step() {
            Ok(step_result) => {
                if step_result.cell == CellState::Unvisited {
                    num_positions += 1;
                }
            }
            Err(_) => break,
        }
    }
    println!("The guard visits {} position(s) on the map.", num_positions);
    Ok(())
}

fn do_part2(state: &State) -> Result<(), Box<dyn Error>> {
    let mut loops = HashSet::new();
    let (_, history) = state.history().split_last().unwrap();
    for guard in history {
        let (i, j) = guard.position;
        let (k, l) = match guard.direction {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        };
        if loops.contains(&(k, l)) {
            continue;
        }
        if state.map()[k][l] == CellState::Obstacle {
            continue;
        }
        let mut map = state.fresh_map();
        let mut temp_state;
        map[k][l] = CellState::Obstacle;
        temp_state = State::new(map);
        temp_state.place_guard(history[0].position);
        loop {
            match temp_state.do_step() {
                Ok(step_result) => {
                    if step_result.loop_detected {
                        loops.insert((k, l));
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }
    println!("{} loop(s) found.", loops.len());
    Ok(())
}

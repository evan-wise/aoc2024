use crate::aoc::{read_lines, Answers, Direction, Position, Solution};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day16 {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
    start: Position,
    end: Position,
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16 {
            width: 0,
            height: 0,
            grid: Vec::new(),
            start: (0, 0),
            end: (0, 0),
        }
    }

    fn next(&self, pos: Position, dir: Direction) -> Option<(Position, Cell)> {
        let (x, y) = pos;
        match dir {
            Direction::Up => {
                if y >= 1 {
                    Some(((x, y - 1), self.grid[y - 1][x]))
                } else {
                    None
                }
            }
            Direction::Down => {
                if y + 1 < self.height {
                    Some(((x, y + 1), self.grid[y + 1][x]))
                } else {
                    None
                }
            }
            Direction::Left => {
                if x >= 1 {
                    Some(((x - 1, y), self.grid[y][x - 1]))
                } else {
                    None
                }
            }
            Direction::Right => {
                if x + 1 < self.width {
                    Some(((x + 1, y), self.grid[y][x + 1]))
                } else {
                    None
                }
            }
        }
    }

    fn minimal_paths(&self) -> (usize, usize) {
        let mut visited = HashSet::new();
        let mut binary_heap =
            BinaryHeap::from([(Reverse(0 as usize), self.start, Direction::Right, None)]);
        let mut low_scores = HashMap::new();
        let mut low_scoring_prevs: HashMap<(Position, Direction), HashSet<(Position, Direction)>> =
            HashMap::new();
        let mut low_score = usize::MAX;
        let mut end_dirs = Vec::new();
        while let Some((Reverse(score), pos, dir, maybe_prev)) = binary_heap.pop() {
            if pos == self.end {
                if score <= low_score {
                    low_score = score;
                    end_dirs.push(dir);
                } else {
                    continue;
                }
            }

            let prev_score = *low_scores.get(&(pos, dir)).unwrap_or(&usize::MAX);
            if score <= prev_score {
                low_scores.insert((pos, dir), score);
                if let Some(prev) = maybe_prev {
                    low_scoring_prevs
                        .entry((pos, dir))
                        .or_insert_with(HashSet::new)
                        .insert(prev);
                }
            }

            if !visited.insert((pos, dir)) {
                continue;
            }

            for (d, s) in [
                (dir, score + 1),
                (dir.right(), score + 1001),
                (dir.left(), score + 1001),
            ] {
                if let Some(((x, y), Cell::Empty)) = self.next(pos, d) {
                    binary_heap.push((Reverse(s), (x, y), d, Some((pos, dir))));
                }
            }
        }

        let mut nodes = HashSet::new();
        let mut visited = HashSet::new();
        let mut stack = end_dirs
            .iter()
            .map(|d| (self.end, *d))
            .collect::<Vec<(Position, Direction)>>();
        while let Some((pos, dir)) = stack.pop() {
            nodes.insert(pos);
            if visited.insert((pos, dir)) {
                let empty = HashSet::new();
                let prevs = low_scoring_prevs.get(&(pos, dir)).unwrap_or(&empty);
                stack.extend(prevs);
            }
        }

        (low_score, nodes.len())
    }
}

impl Solution for Day16 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day16.txt")?;
        for (y, line) in lines.flatten().enumerate() {
            let chars = line.chars().collect::<Vec<char>>();
            self.grid.push(
                chars
                    .iter()
                    .map(|&c| Cell::parse(c))
                    .collect::<Result<_, _>>()?,
            );
            for (x, &c) in chars.iter().enumerate() {
                if c == 'S' {
                    self.start = (x, y);
                }
                if c == 'E' {
                    self.end = (x, y);
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
        let (score, num_seats) = self.minimal_paths();
        Ok(Answers::both(score, num_seats))
    }
}

// For debugging
#[allow(dead_code)]
fn print_map(map: &Day16, nodes: &HashSet<Position>) {
    for (y, row) in map.grid.iter().enumerate() {
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(x, c)| if (x, y) == map.end {
                    "E".to_string()
                } else if (x, y) == map.start {
                    "S".to_string()
                } else if nodes.contains(&(x, y)) {
                    "O".to_string()
                } else {
                    format!("{c}")
                })
                .collect::<String>()
        );
    }
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Empty,
    Wall,
}

impl Cell {
    fn parse(c: char) -> Result<Cell, String> {
        match c {
            '#' => Ok(Cell::Wall),
            '.' | 'S' | 'E' => Ok(Cell::Empty),
            _ => Err("invalid character".to_string()),
        }
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Empty => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day16::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(109496usize, 551usize));
        Ok(())
    }
}

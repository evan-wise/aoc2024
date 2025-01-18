use crate::aoc::grid::Grid;
use crate::aoc::{read_lines, Answers, Map, Position, Solution};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day18 {
    grid: Grid<Cell>,
    bytes: Vec<Position>,
    num_bytes: usize,
    size: usize,
}

impl Day18 {
    pub fn new() -> Day18 {
        Day18 {
            grid: Grid::new(),
            bytes: Vec::new(),
            num_bytes: 0,
            size: 0,
        }
    }
}

impl Solution for Day18 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day18.txt";
        self.num_bytes = if filename.contains("/data/") {
            1024
        } else if filename.contains("/examples/") {
            12
        } else {
            return Err("expected path to contain \"/data/\" or \"/examples/\"".into());
        };
        self.size = if self.num_bytes == 1024 { 71 } else { 7 };
        self.grid = Grid::fill(Cell::Safe, self.size, self.size);

        let lines = read_lines(filename)?;
        for (line_num, line) in lines.flatten().enumerate() {
            let parts = line
                .split(",")
                .map(|p| p.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;
            if parts.len() != 2 {
                return Err(format!("invalid line {line_num}").into());
            }
            self.bytes.push((parts[0], parts[1]));
        }
        if self.bytes.len() < self.num_bytes {
            return Err(format!("byte stream has less than {} bytes", self.num_bytes).into());
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        for i in 0..self.num_bytes {
            let byte = self.bytes[i];
            self.grid[byte] = Cell::Corrupted;
        }
        let start = (0, 0);
        let end = (self.size - 1, self.size - 1);
        let (dist, _) = self.minimal_path(Cell::Safe, start, end);
        let mut byte_str = String::new();
        for i in self.num_bytes..self.bytes.len() {
            let byte = self.bytes[i];
            self.grid[byte] = Cell::Corrupted;
            if let Some(true) = self.chokepoint(byte) {
                if let (None, _) = self.minimal_path(Cell::Safe, start, end) {
                    byte_str = format!("{},{}", byte.0, byte.1);
                    break;
                }
            }
        }
        Ok(Answers::both(dist.unwrap(), byte_str))
    }
}

impl Map for Day18 {
    type Cell = Cell;

    fn width(&self) -> usize {
        self.size
    }

    fn height(&self) -> usize {
        self.size
    }

    fn get(&self, pos: &Position) -> Option<&Cell> {
        self.grid.get(&pos)
    }
}

impl Day18 {
    fn chokepoint(&self, pos: Position) -> Option<bool> {
        let (x, y) = pos;
        if x >= self.size || y >= self.size {
            return None;
        }
        let t = if y == 0 {
            true
        } else {
            match self.grid[(x, y - 1)] {
                Cell::Corrupted => true,
                _ => false,
            }
        };
        let b = if y == self.size - 1 {
            true
        } else {
            match self.grid[(x, y + 1)] {
                Cell::Corrupted => true,
                _ => false,
            }
        };
        let l = if x == 0 {
            true
        } else {
            match self.grid[(x - 1, y)] {
                Cell::Corrupted => true,
                _ => false,
            }
        };
        let r = if x == self.size - 1 {
            true
        } else {
            match self.grid[(x + 1, y)] {
                Cell::Corrupted => true,
                _ => false,
            }
        };
        let tl = if x == 0 || y == 0 {
            true
        } else {
            match self.grid[(x - 1, y - 1)] {
                Cell::Corrupted => true,
                _ => false,
            }
        };
        let tr = if x == self.size - 1 || y == 0 {
            true
        } else {
            match self.grid[(x + 1, y - 1)] {
                Cell::Corrupted => true,
                _ => false,
            }
        };
        let bl = if x == 0 || y == self.size - 1 {
            true
        } else {
            match self.grid[(x - 1, y + 1)] {
                Cell::Corrupted => true,
                _ => false,
            }
        };
        let br = if x == self.size - 1 || y == self.size - 1 {
            true
        } else {
            match self.grid[(x + 1, y + 1)] {
                Cell::Corrupted => true,
                _ => false,
            }
        };

        Some(
            (t && b)
                || (l && r)
                || (tl && br)
                || (tr && bl)
                || (tl && tr)
                || (tr && br)
                || (br && bl)
                || (bl && tl),
        )
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Cell {
    Safe,
    Corrupted,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Safe => write!(f, "."),
            Self::Corrupted => write!(f, "#"),
        }
    }
}

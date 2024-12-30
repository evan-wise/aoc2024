use crate::aoc::{read_lines, Answers, Direction, Map, Position, Solution};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;

pub struct Day18;

impl Solution for Day18 {
    type Part1 = usize;
    type Part2 = String;

    fn solve(&self) -> Result<Answers<Self::Part1, Self::Part2>, Box<dyn Error>> {
        let (bytes, num_bytes, map_size) = parse_input("./data/day18.txt")?;
        let mut map = Day18Map::new(map_size);
        for i in 0..num_bytes {
            map.corrupted.insert(bytes[i]);
        }
        let dist = minimal_path(&map);
        Answers::ok(Some(dist), None)
    }
}

fn parse_input(filename: &str) -> Result<(Vec<Position>, usize, usize), Box<dyn Error>> {
    let lines = read_lines(filename)?;
    let num_bytes = if filename.contains("/data/") {
        1024
    } else if filename.contains("/examples/") {
        12
    } else {
        return Err("expected path to contain \"/data/\" or \"/examples/\"".into());
    };
    let map_size = if num_bytes == 1024 { 71 } else { 7 };
    let mut bytes = Vec::new();
    for (line_num, line) in lines.flatten().enumerate() {
        let parts = line
            .split(",")
            .map(|p| p.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        if parts.len() != 2 {
            return Err(format!("invalid line {line_num}").into());
        }
        bytes.push((parts[0], parts[1]));
    }
    if bytes.len() < num_bytes {
        return Err(format!("byte stream has less than {num_bytes} bytes").into());
    }
    Ok((bytes, num_bytes, map_size))
}

fn minimal_path(map: &Day18Map) -> usize {
    let start = (0, 0);
    let end = (map.width() - 1, map.height() - 1);
    let mut heap = BinaryHeap::from([(Reverse(0), start)]);
    let mut visited = HashSet::new();
    let mut low_dists = HashMap::new();
    while let Some((Reverse(dist), pos)) = heap.pop() {
        let prev_dist = *low_dists.get(&pos).unwrap_or(&usize::MAX);

        if dist >= prev_dist {
            continue;
        }
        low_dists.insert(pos, dist);
        if !visited.insert(pos) {
            continue;
        }
        if pos == end {
            continue;
        }
        for d in Direction::all() {
            if let Some(((x, y), Cell::Safe)) = d.go(map, pos) {
                heap.push((Reverse(dist + 1), (x, y)));
            }
        }
    }
    *low_dists.get(&end).unwrap()
}

#[derive(Debug)]
struct Day18Map {
    width: usize,
    height: usize,
    corrupted: HashSet<Position>,
}

impl Map for Day18Map {
    type Cell = Cell;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, pos: Position) -> Option<Cell> {
        let (x, y) = pos;
        if x >= self.width || y >= self.height {
            return None;
        }
        if self.corrupted.contains(&pos) {
            Some(Cell::Corrupted)
        } else {
            Some(Cell::Safe)
        }
    }
}

impl Day18Map {
    fn new(size: usize) -> Day18Map {
        Day18Map {
            width: size,
            height: size,
            corrupted: HashSet::new(),
        }
    }
}

#[derive(Debug)]
enum Cell {
    Safe,
    Corrupted,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Safe => write!(f, "."),
            Self::Corrupted => write!(f, "#"),
        }
    }
}

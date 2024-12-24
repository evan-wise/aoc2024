use crate::aoc::{read_lines, Direction, Position, Solution, SolutionParts};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::path::Path;

pub struct Day16;

impl Solution for Day16 {
    fn solve(&self) -> Result<SolutionParts, Box<dyn Error>> {
        let map = parse_input("./data/day16.txt")?;
        let (score, num_seats) = minimal_path(&map);
        Ok((Some(score.to_string()), Some(num_seats.to_string())))
    }
}

fn parse_input<P: AsRef<Path>>(path: P) -> Result<Map, Box<dyn Error>> {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let lines = read_lines(path)?;
    for (y, line) in lines.flatten().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();
        grid.push(
            chars
                .iter()
                .map(|&c| Cell::parse(c).unwrap())
                .collect::<Vec<Cell>>(),
        );
        for (x, &c) in chars.iter().enumerate() {
            if c == 'S' {
                start = (x, y);
            }
            if c == 'E' {
                end = (x, y);
            }
        }
    }
    Ok(Map::new(grid, start, end))
}

fn minimal_path(map: &Map) -> (usize, usize) {
    let mut visited = HashSet::new();
    let mut binary_heap = BinaryHeap::from([(Reverse(0 as usize), map.start, Direction::Right)]);
    let mut scores = Vec::new();
    let mut predecessors_by_pos: HashMap<Position, HashSet<Position>> = HashMap::new();
    while let Some((Reverse(score), pos, dir)) = binary_heap.pop() {
        if pos == map.end {
            scores.push(score);
        }

        if visited.insert((pos, dir)) {
            for (d, s) in [
                (dir, score + 1),
                (dir.right(), score + 1001),
                (dir.left(), score + 1001),
            ] {
                if let Some((x, y)) = map.next_pos(pos, d) {
                    if let Cell::Empty = map.grid[y][x] {
                        binary_heap.push((Reverse(s), (x, y), d));
                        predecessors_by_pos
                            .entry((x, y))
                            .or_insert_with(HashSet::new)
                            .insert(pos);
                    }
                }
            }
        }
    }

    // This is wrong, I am counting ALL the paths not just the minimal ones...
    // Not sure how to modify this at the moment, gonna come back to it.
    let mut nodes = HashSet::new();
    let mut stack = vec![map.end];
    while let Some(pos) = stack.pop() {
        if nodes.insert(pos) {
            let empty = HashSet::new();
            let predecessors = predecessors_by_pos.get(&pos).unwrap_or(&empty);
            stack.extend(predecessors);
        }
    }

    (*scores.iter().min().unwrap_or(&0), nodes.len())
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
    start: Position,
    end: Position,
}

impl Map {
    fn new(grid: Vec<Vec<Cell>>, start: Position, end: Position) -> Map {
        let height = grid.len();
        let mut width = 0;
        if height != 0 {
            width = grid[0].len();
        }
        Map {
            width,
            height,
            grid,
            start,
            end,
        }
    }

    fn next_pos(&self, pos: Position, dir: Direction) -> Option<Position> {
        let (x, y) = pos;
        match dir {
            Direction::Up => {
                if y >= 1 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if y + 1 < self.height {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if x >= 1 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Direction::Right => {
                if x + 1 < self.width {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
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

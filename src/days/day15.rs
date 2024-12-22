use crate::aoc::read_lines;
use crate::days::Solution;
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::path::Path;

pub struct Day15;

impl Solution for Day15 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        let (mut map, mut robot) = parse_input("./data/day15.txt")?;
        let mut expanded_map = map.expanded()?;
        let mut other_robot = robot.clone();
        other_robot.pos.0 *= 2;
        while let Some(_) = robot.do_move(&mut map) {}
        println!("Part 1: {}", map.compute_sum());
        while let Some(_) = other_robot.do_move(&mut expanded_map) {}
        println!("Part 2: {}", expanded_map.compute_sum());
        Ok(())
    }
}

fn parse_input<P: AsRef<Path>>(path: P) -> Result<(Map, Robot), io::Error> {
    let lines = read_lines(path)?;
    let mut before_break = true;
    let mut width = 0;
    let mut height = 0;
    let mut grid = Vec::new();
    let mut pos = (0, 0);
    let mut moves = Vec::new();
    for line in lines.flatten() {
        if before_break {
            if line == "" {
                before_break = false;
                continue;
            }
            if let Some(x) = line.chars().position(|c| c == '@') {
                pos = (x, height);
            }
            let row = line
                .chars()
                .map(|c| Cell::parse(c))
                .flatten()
                .collect::<Vec<Cell>>();
            if width == 0 {
                width = row.len();
            }
            grid.push(row);
            height += 1;
        } else {
            moves.extend(line.chars().map(|c| Direction::parse(c)).flatten());
        }
    }
    Ok((
        Map {
            width,
            height,
            grid,
            robot_pos: pos,
        },
        Robot {
            pos,
            moves,
            move_num: 0,
        },
    ))
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Empty,
}

impl Cell {
    fn parse(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Empty),
            _ => Err("invalid character \"{c}\" in map".to_string()),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Wall => write!(f, "#"),
            Cell::Box => write!(f, "O"),
            Cell::BoxLeft => write!(f, "["),
            Cell::BoxRight => write!(f, "]"),
            Cell::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct Map {
    height: usize,
    width: usize,
    grid: Vec<Vec<Cell>>,
    robot_pos: Position,
}

impl Map {
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

    fn compute_sum(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if let Cell::Box | Cell::BoxLeft = self.grid[y][x] {
                    sum += 100 * y + x;
                }
            }
        }
        sum
    }

    fn expanded(&self) -> Result<Map, String> {
        let mut grid = Vec::new();
        for y in 0..self.height {
            let mut row = Vec::new();
            for x in 0..self.width {
                match self.grid[y][x] {
                    Cell::Empty => {
                        row.extend([Cell::Empty, Cell::Empty]);
                    }
                    Cell::Wall => {
                        row.extend([Cell::Wall, Cell::Wall]);
                    }
                    Cell::Box => {
                        row.extend([Cell::BoxLeft, Cell::BoxRight]);
                    }
                    _ => {
                        return Err("cannot expand already expanded map".to_string());
                    }
                }
            }
            grid.push(row);
        }
        let (x, y) = self.robot_pos;
        Ok(Map {
            height: self.height,
            width: self.width * 2,
            grid,
            robot_pos: (2 * x, y),
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .enumerate()
                .map(|(y, r)| r
                    .iter()
                    .enumerate()
                    .map(|(x, c)| if (x, y) == self.robot_pos {
                        "@".to_string()
                    } else {
                        format!("{c}")
                    })
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

type Position = (usize, usize);

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Result<Self, String> {
        match c {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err("invalid character \"{c}\" in moves".to_string()),
        }
    }
}

#[derive(Clone, Debug)]
struct Robot {
    pos: Position,
    moves: Vec<Direction>,
    move_num: usize,
}

impl Robot {
    fn do_move(&mut self, map: &mut Map) -> Option<Position> {
        if self.move_num >= self.moves.len() {
            return None;
        }
        let dir = self.moves[self.move_num];
        if let Some(next_pos) = map.next_pos(self.pos, dir) {
            let (x, y) = next_pos;
            match map.grid[y][x] {
                Cell::Empty => {
                    self.pos = next_pos;
                }
                Cell::Box => {
                    if self.push_small_box(map, dir, next_pos) {
                        self.pos = next_pos;
                    }
                }
                Cell::BoxLeft | Cell::BoxRight => {
                    if self.can_push_large_box(map, dir, next_pos) {
                        self.push_large_box(map, dir, next_pos);
                        self.pos = next_pos;
                    }
                }
                Cell::Wall => (),
            }
        }
        self.move_num += 1;
        map.robot_pos = self.pos;
        Some(self.pos)
    }

    fn push_small_box(&mut self, map: &mut Map, dir: Direction, pos: Position) -> bool {
        let mut next_pos = pos;
        while let Cell::Box = map.grid[next_pos.1][next_pos.0] {
            next_pos = map.next_pos(next_pos, dir).unwrap();
        }
        if let Cell::Empty = map.grid[next_pos.1][next_pos.0] {
            map.grid[next_pos.1][next_pos.0] = Cell::Box;
            map.grid[pos.1][pos.0] = Cell::Empty;
            true
        } else {
            false
        }
    }

    fn can_push_large_box(&mut self, map: &mut Map, dir: Direction, pos: Position) -> bool {
        let (x, y) = pos;
        if let Direction::Up | Direction::Down = dir {
            let (next_pos_left, next_pos_right) = match map.grid[y][x] {
                Cell::BoxLeft => (
                    map.next_pos(pos, dir).unwrap(),
                    map.next_pos(((x + 1), y), dir).unwrap(),
                ),
                Cell::BoxRight => (
                    map.next_pos(((x - 1), y), dir).unwrap(),
                    map.next_pos(pos, dir).unwrap(),
                ),
                _ => {
                    return false;
                }
            };
            let (xl, yl) = next_pos_left;
            let (xr, yr) = next_pos_right;
            match (map.grid[yl][xl], map.grid[yr][xr]) {
                (Cell::Empty, Cell::Empty) => true,
                (Cell::BoxRight, Cell::Empty) => {
                    if self.can_push_large_box(map, dir, (xl, yl)) {
                        true
                    } else {
                        false
                    }
                }
                (Cell::Empty, Cell::BoxLeft) => {
                    if self.can_push_large_box(map, dir, (xr, yr)) {
                        true
                    } else {
                        false
                    }
                }
                (Cell::BoxLeft, Cell::BoxRight) => {
                    if self.can_push_large_box(map, dir, (xl, yl)) {
                        true
                    } else {
                        false
                    }
                }
                (Cell::BoxRight, Cell::BoxLeft) => {
                    let left_flag = self.can_push_large_box(map, dir, (xl, yl));
                    let right_flag = self.can_push_large_box(map, dir, (xr, yr));
                    if left_flag && right_flag {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            match map.grid[y][x] {
                Cell::BoxLeft | Cell::BoxRight => {
                    let (xn, yn) = map.next_pos(pos, dir).unwrap();
                    match map.grid[yn][xn] {
                        Cell::Empty => true,
                        Cell::BoxLeft | Cell::BoxRight => {
                            if self.can_push_large_box(map, dir, (xn, yn)) {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                }
                _ => false,
            }
        }
    }

    fn push_large_box(&mut self, map: &mut Map, dir: Direction, pos: Position) {
        let (x, y) = pos;
        if let Direction::Up | Direction::Down = dir {
            let (next_pos_left, next_pos_right, offset_x) = match map.grid[y][x] {
                Cell::BoxLeft => (
                    map.next_pos(pos, dir).unwrap(),
                    map.next_pos(((x + 1), y), dir).unwrap(),
                    x + 1,
                ),
                Cell::BoxRight => (
                    map.next_pos(((x - 1), y), dir).unwrap(),
                    map.next_pos(pos, dir).unwrap(),
                    x - 1,
                ),
                _ => {
                    return;
                }
            };
            let (xl, yl) = next_pos_left;
            let (xr, yr) = next_pos_right;
            match (map.grid[yl][xl], map.grid[yr][xr]) {
                (Cell::BoxRight, Cell::Empty) => self.push_large_box(map, dir, (xl, yl)),
                (Cell::Empty, Cell::BoxLeft) => self.push_large_box(map, dir, (xr, yr)),

                (Cell::BoxLeft, Cell::BoxRight) => self.push_large_box(map, dir, (xl, yl)),
                (Cell::BoxRight, Cell::BoxLeft) => {
                    self.push_large_box(map, dir, (xl, yl));
                    self.push_large_box(map, dir, (xr, yr));
                }
                _ => (),
            }
            map.grid[yl][xl] = Cell::BoxLeft;
            map.grid[yr][xr] = Cell::BoxRight;
            map.grid[y][x] = Cell::Empty;
            map.grid[y][offset_x] = Cell::Empty;
        } else {
            if let Cell::BoxLeft | Cell::BoxRight = map.grid[y][x] {
                let (xn, yn) = map.next_pos(pos, dir).unwrap();
                if let Cell::BoxLeft | Cell::BoxRight = map.grid[yn][xn] {
                    self.push_large_box(map, dir, (xn, yn));
                }
                map.grid[yn][xn] = map.grid[y][x];
                map.grid[y][x] = Cell::Empty;
            }
        }
    }
}

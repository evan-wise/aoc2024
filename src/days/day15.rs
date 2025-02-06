use crate::aoc::{read_lines, Answers, Solution};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day15 {
    robot: Robot,
    grid: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 {
            robot: Robot::new(),
            grid: Vec::new(),
            width: 0,
            height: 0,
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

    fn expanded(&self) -> Result<Vec<Vec<Cell>>, String> {
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
        Ok(grid)
    }

    fn do_move(&mut self) -> Option<Position> {
        if self.robot.move_num >= self.robot.moves.len() {
            return None;
        }
        let dir = self.robot.moves[self.robot.move_num];
        if let Some(next_pos) = self.next_pos(self.robot.pos, dir) {
            let (x, y) = next_pos;
            match self.grid[y][x] {
                Cell::Empty => {
                    self.robot.pos = next_pos;
                }
                Cell::Box => {
                    if self.push_small_box(dir, next_pos) {
                        self.robot.pos = next_pos;
                    }
                }
                Cell::BoxLeft | Cell::BoxRight => {
                    if self.can_push_large_box(dir, next_pos) {
                        self.push_large_box(dir, next_pos);
                        self.robot.pos = next_pos;
                    }
                }
                Cell::Wall => (),
            }
        }
        self.robot.move_num += 1;
        Some(self.robot.pos)
    }

    fn push_small_box(&mut self, dir: Direction, pos: Position) -> bool {
        let mut next_pos = pos;
        while let Cell::Box = self.grid[next_pos.1][next_pos.0] {
            next_pos = self.next_pos(next_pos, dir).unwrap();
        }
        if let Cell::Empty = self.grid[next_pos.1][next_pos.0] {
            self.grid[next_pos.1][next_pos.0] = Cell::Box;
            self.grid[pos.1][pos.0] = Cell::Empty;
            true
        } else {
            false
        }
    }

    fn can_push_large_box(&mut self, dir: Direction, pos: Position) -> bool {
        let (x, y) = pos;
        if let Direction::Up | Direction::Down = dir {
            let (next_pos_left, next_pos_right) = match self.grid[y][x] {
                Cell::BoxLeft => (
                    self.next_pos(pos, dir).unwrap(),
                    self.next_pos(((x + 1), y), dir).unwrap(),
                ),
                Cell::BoxRight => (
                    self.next_pos(((x - 1), y), dir).unwrap(),
                    self.next_pos(pos, dir).unwrap(),
                ),
                _ => {
                    return false;
                }
            };
            let (xl, yl) = next_pos_left;
            let (xr, yr) = next_pos_right;
            match (self.grid[yl][xl], self.grid[yr][xr]) {
                (Cell::Empty, Cell::Empty) => true,
                (Cell::BoxRight, Cell::Empty) => {
                    if self.can_push_large_box(dir, (xl, yl)) {
                        true
                    } else {
                        false
                    }
                }
                (Cell::Empty, Cell::BoxLeft) => {
                    if self.can_push_large_box(dir, (xr, yr)) {
                        true
                    } else {
                        false
                    }
                }
                (Cell::BoxLeft, Cell::BoxRight) => {
                    if self.can_push_large_box(dir, (xl, yl)) {
                        true
                    } else {
                        false
                    }
                }
                (Cell::BoxRight, Cell::BoxLeft) => {
                    let left_flag = self.can_push_large_box(dir, (xl, yl));
                    let right_flag = self.can_push_large_box(dir, (xr, yr));
                    if left_flag && right_flag {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            match self.grid[y][x] {
                Cell::BoxLeft | Cell::BoxRight => {
                    let (xn, yn) = self.next_pos(pos, dir).unwrap();
                    match self.grid[yn][xn] {
                        Cell::Empty => true,
                        Cell::BoxLeft | Cell::BoxRight => {
                            if self.can_push_large_box(dir, (xn, yn)) {
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

    fn push_large_box(&mut self, dir: Direction, pos: Position) {
        let (x, y) = pos;
        if let Direction::Up | Direction::Down = dir {
            let (next_pos_left, next_pos_right, offset_x) = match self.grid[y][x] {
                Cell::BoxLeft => (
                    self.next_pos(pos, dir).unwrap(),
                    self.next_pos(((x + 1), y), dir).unwrap(),
                    x + 1,
                ),
                Cell::BoxRight => (
                    self.next_pos(((x - 1), y), dir).unwrap(),
                    self.next_pos(pos, dir).unwrap(),
                    x - 1,
                ),
                _ => {
                    return;
                }
            };
            let (xl, yl) = next_pos_left;
            let (xr, yr) = next_pos_right;
            match (self.grid[yl][xl], self.grid[yr][xr]) {
                (Cell::BoxRight, Cell::Empty) => self.push_large_box(dir, (xl, yl)),
                (Cell::Empty, Cell::BoxLeft) => self.push_large_box(dir, (xr, yr)),

                (Cell::BoxLeft, Cell::BoxRight) => self.push_large_box(dir, (xl, yl)),
                (Cell::BoxRight, Cell::BoxLeft) => {
                    self.push_large_box(dir, (xl, yl));
                    self.push_large_box(dir, (xr, yr));
                }
                _ => (),
            }
            self.grid[yl][xl] = Cell::BoxLeft;
            self.grid[yr][xr] = Cell::BoxRight;
            self.grid[y][x] = Cell::Empty;
            self.grid[y][offset_x] = Cell::Empty;
        } else {
            if let Cell::BoxLeft | Cell::BoxRight = self.grid[y][x] {
                let (xn, yn) = self.next_pos(pos, dir).unwrap();
                if let Cell::BoxLeft | Cell::BoxRight = self.grid[yn][xn] {
                    self.push_large_box(dir, (xn, yn));
                }
                self.grid[yn][xn] = self.grid[y][x];
                self.grid[y][x] = Cell::Empty;
            }
        }
    }
}

impl Solution for Day15 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day15.txt")?;
        let mut before_break = true;
        let mut width = 0;
        let mut height = 0;
        for line in lines.flatten() {
            if before_break {
                if line == "" {
                    before_break = false;
                    continue;
                }
                if let Some(x) = line.chars().position(|c| c == '@') {
                    self.robot.pos = (x, height);
                }
                let row = line
                    .chars()
                    .map(Cell::parse)
                    .collect::<Result<Vec<_>, _>>()?;
                if width == 0 {
                    width = row.len();
                }
                self.grid.push(row);
                height += 1;
            } else {
                self.robot
                    .moves
                    .extend(line.chars().map(|c| Direction::parse(c)).flatten());
            }
        }
        self.height = height;
        self.width = width;
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let expanded_grid = self.expanded()?;
        let (x, y) = self.robot.pos;

        while let Some(_) = self.do_move() {}
        let sum1 = self.compute_sum();

        self.grid = expanded_grid;
        self.width *= 2;
        self.robot.pos = (2 * x, y);
        self.robot.move_num = 0;

        while let Some(_) = self.do_move() {}
        let sum2 = self.compute_sum();
        Ok(Answers::both(sum1, sum2))
    }
}

impl Display for Day15 {
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
                    .map(|(x, c)| if (x, y) == self.robot.pos {
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
    fn new() -> Robot {
        Robot {
            pos: (0, 0),
            moves: Vec::new(),
            move_num: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day15::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(1478649usize, 1495455usize));
        Ok(())
    }
}

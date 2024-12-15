use crate::aoc::read_lines;
use crate::days::Solution;
use std::error::Error;
use std::io;
use std::path::Path;

pub struct Day15;

impl Solution for Day15 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        let (mut map, mut robot) = parse_input("./examples/day15.txt")?;
        println!("{:?}", map);
        println!("{:?}", robot);
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
                pos = (x as i32, height);
            }
            let row = line.chars().map(|c| Cell::parse(c)).flatten().collect::<Vec<Cell>>();
            if width == 0 {
                width = row.len() as i32;
            }
            grid.push(row);
            height += 1;
        } else {
            moves.extend(line.chars().map(|c| Direction::parse(c)).flatten());
        }
    }

    Ok((Map { width, height, grid }, Robot { pos, moves }))
}

#[derive(Debug)]
enum Cell {
    Wall,
    Box,
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

#[derive(Debug)]
struct Map {
    height: i32,
    width: i32,
    grid: Vec<Vec<Cell>>,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    moves: Vec<Direction>,
}


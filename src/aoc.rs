use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Read};
use std::path::Path;

pub struct Answers<T: Display, U: Display> {
    part1: Option<T>,
    part2: Option<U>,
}

impl<T: Display, U: Display> Display for Answers<T, U> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(part1) = &self.part1 {
            write!(f, "Part 1: {part1}")?;
            if let Some(_) = &self.part2 {
                write!(f, "\n")?;
            }
        }
        if let Some(part2) = &self.part2 {
            write!(f, "Part 2: {part2}")?;
        }
        Ok(())
    }
}

impl<T: Display, U: Display> Answers<T, U> {
    pub fn ok(part1: Option<T>, part2: Option<U>) -> Result<Answers<T, U>, Box<dyn Error>> {
        Ok(Answers { part1, part2 })
    }
}

pub trait Solution {
    type Part1: Display;
    type Part2: Display;
    fn solve(&self) -> Result<Answers<Self::Part1, Self::Part2>, Box<dyn Error>>;
}

pub trait SolutionWrapper {
    fn solve_string(&self) -> Result<String, Box<dyn Error>>;
}

impl<T> SolutionWrapper for T
where
    T: Solution,
{
    fn solve_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(format!("{}", self.solve()?))
    }
}

pub struct FileCharIterator {
    reader: BufReader<File>,
}

impl FileCharIterator {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(FileCharIterator { reader })
    }
}

impl Iterator for FileCharIterator {
    type Item = io::Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => Some(Ok(buf[0] as char)),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn read_chars<P>(path: P) -> io::Result<FileCharIterator>
where
    P: AsRef<Path>,
{
    FileCharIterator::new(path)
}

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub type Position = (usize, usize);

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn right(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    pub fn left(&self) -> Direction {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    pub fn go<T: Map>(&self, map: &T, pos: Position) -> Option<(Position, T::Cell)> {
        let (x, y) = pos;
        let width = map.width();
        let height = map.height();
        let new = match self {
            Self::Up => {
                if y == 0 {
                    return None;
                }
                (x, y - 1)
            }
            Self::Down => {
                if y == height - 1 {
                    return None;
                }
                (x, y + 1)
            }
            Self::Left => {
                if x == 0 {
                    return None;
                }
                (x - 1, y)
            }
            Self::Right => {
                if x == width - 1 {
                    return None;
                }
                (x + 1, y)
            }
        };
        if let Some(state) = map.get(new) {
            return Some((new, state));
        }
        None
    }
}

pub trait Map {
    type Cell: Display;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, pos: Position) -> Option<Self::Cell>;
}

pub struct MapDisplay<'a, T: Map>(pub &'a T);

impl<'a, T: Map> Display for MapDisplay<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let height = self.0.height();
        let width = self.0.width();
        for y in 0..height {
            for x in 0..width {
                write!(f, "{}", self.0.get((x, y)).unwrap())?;
            }
            if y != height - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

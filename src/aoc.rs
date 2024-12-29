use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Read};
use std::path::Path;

pub type SolutionParts = (Option<String>, Option<String>);

pub trait Solution {
    fn solve(&self) -> Result<SolutionParts, Box<dyn Error>>;
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

    pub fn go<T, U: Map<T>>(&self, map: &U, pos: Position) -> Option<(Position, T)> {
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

pub trait Map<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, pos: Position) -> Option<T>;
}

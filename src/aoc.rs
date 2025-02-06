pub mod grid;

use grid::Grid;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Read};
use std::path::Path;
use std::time::{Duration, Instant};

pub trait Solution: Debug {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>>;
    fn solve(&mut self) -> Result<Answers, Box<dyn Error>>;
    fn run(&mut self, num: usize) -> Result<SolutionData, Box<dyn Error>> {
        let parse_timer = Instant::now();
        self.parse_input()?;
        let parse_time = parse_timer.elapsed();

        let solution_timer = Instant::now();
        let answers = self.solve()?;
        let solution_time = solution_timer.elapsed();

        Ok(SolutionData::new(num, answers, parse_time, solution_time))
    }
}

// Suppress warnings since Part1 and None will only be used early in solutions.
#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Answers {
    Both(String, String),
    Part1(String),
    None,
}

impl Display for Answers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Both(part1, part2) => write!(f, "Part 1: {part1}\nPart 2: {part2}"),
            Self::Part1(part1) => write!(f, "Part 1: {part1}"),
            Self::None => Ok(()),
        }
    }
}

// Suppress warnings since Part1 and None will only be used early in solutions.
#[allow(dead_code)]
impl Answers {
    pub fn both<T: Display, U: Display>(part1: T, part2: U) -> Answers {
        Answers::Both(format!("{part1}"), format!("{part2}"))
    }

    pub fn part1<T: Display>(part1: T) -> Answers {
        Answers::Part1(format!("{part1}"))
    }

    pub fn complete(&self) -> bool {
        match self {
            Self::Both(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct SolutionData {
    pub num: usize,
    pub answers: Answers,
    pub parse_time: Duration,
    pub solve_time: Duration,
}

impl SolutionData {
    pub fn new(
        num: usize,
        answers: Answers,
        parse_time: Duration,
        solve_time: Duration,
    ) -> SolutionData {
        SolutionData {
            num,
            answers,
            parse_time,
            solve_time,
        }
    }
}

impl Display for SolutionData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let parse_millis = self.parse_time.as_micros() as f64 / 1000.0;
        let solve_millis = self.solve_time.as_micros() as f64 / 1000.0;
        write!(f, "~- DAY {:0>2} -~\n", self.num)?;
        write!(
            f,
            "Parse: {parse_millis:.3}ms, Solve: {solve_millis:.3}ms\n"
        )?;
        write!(f, "{}", self.answers)
    }
}

pub struct Statistics<'a> {
    pub total_time: Duration,
    pub complete: Vec<&'a SolutionData>,
    pub avg: f64,
    pub stddev: f64,
}

impl Statistics<'_> {
    pub fn calc(data: &Vec<SolutionData>) -> Statistics {
        let total_time = data.iter().fold(Duration::from_secs(0), |a, s| {
            a + s.parse_time + s.solve_time
        });
        let complete: Vec<_> = data.iter().filter(|s| s.answers.complete()).collect();
        let completed = complete.len();
        let avg = complete
            .iter()
            .fold(Duration::from_secs(0), |a, s| {
                a + s.parse_time + s.solve_time
            })
            .as_micros() as f64
            / completed as f64;
        let stddev = complete
            .iter()
            .fold(0.0, |a, s| {
                a + (s.parse_time.as_micros() as f64 + s.solve_time.as_micros() as f64 - avg)
                    .powf(2.0)
            })
            .sqrt()
            / completed as f64;
        Statistics {
            total_time,
            complete,
            avg,
            stddev,
        }
    }
}

impl Display for Statistics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "~- STATISTICS -~\n")?;
        write!(
            f,
            "T = {}ms, n = {}, μ = {:.2}ms, σ = {:.2}ms",
            self.total_time.as_millis(),
            self.complete.len(),
            self.avg / 1000.0,
            self.stddev / 1000.0
        )
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
}

pub trait Map {
    type Cell: Display + Eq;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, pos: &Position) -> Option<&Self::Cell>;

    fn minimal_path(
        &self,
        open: Self::Cell,
        start: Position,
        end: Position,
    ) -> (Option<usize>, Grid<usize>) {
        let mut heap = BinaryHeap::from([(Reverse(0), start)]);
        let mut lows = Grid::fill(usize::MAX, self.width(), self.height());
        while let Some((Reverse(dist), pos)) = heap.pop() {
            let prev_dist = *lows.get(&pos).unwrap_or(&usize::MAX);
            if dist > prev_dist {
                continue;
            }
            if prev_dist < usize::MAX {
                continue;
            }
            lows[pos] = dist;
            if pos == end {
                continue;
            }
            for d in Direction::all() {
                if let Some(((x, y), cell)) = self.go(d, &pos) {
                    if *cell == open {
                        heap.push((Reverse(dist + 1), (x, y)));
                    }
                }
            }
        }
        (lows.get(&end).copied().filter(|l| *l != usize::MAX), lows)
    }

    fn backtrack_minimal_path(
        &self,
        open: Self::Cell,
        start: Position,
        end: Position,
    ) -> (
        Option<usize>,
        Grid<usize>,
        FxHashMap<Position, FxHashSet<Position>>,
    ) {
        let mut heap = BinaryHeap::from([(Reverse(0), start, None)]);
        let mut lows = Grid::fill(usize::MAX, self.width(), self.height());
        let mut backtracks = FxHashMap::default();
        while let Some((Reverse(dist), pos, maybe_prev)) = heap.pop() {
            let prev_dist = *lows.get(&pos).unwrap_or(&usize::MAX);
            if dist > prev_dist {
                continue;
            }
            if let Some(prev) = maybe_prev {
                backtracks
                    .entry(pos)
                    .or_insert_with(FxHashSet::default)
                    .insert(prev);
            }
            if prev_dist < usize::MAX {
                continue;
            }
            lows[pos] = dist;
            if pos == end {
                continue;
            }
            for d in Direction::all() {
                if let Some(((x, y), cell)) = self.go(d, &pos) {
                    if *cell == open {
                        heap.push((Reverse(dist + 1), (x, y), Some(pos)));
                    }
                }
            }
        }
        (lows.get(&end).copied(), lows, backtracks)
    }

    fn go(&self, dir: Direction, pos: &Position) -> Option<(Position, &Self::Cell)> {
        let (x, y) = *pos;
        let width = self.width();
        let height = self.height();
        let new = match dir {
            Direction::Up => {
                if y == 0 {
                    return None;
                }
                (x, y - 1)
            }
            Direction::Down => {
                if y == height - 1 {
                    return None;
                }
                (x, y + 1)
            }
            Direction::Left => {
                if x == 0 {
                    return None;
                }
                (x - 1, y)
            }
            Direction::Right => {
                if x == width - 1 {
                    return None;
                }
                (x + 1, y)
            }
        };
        if let Some(state) = self.get(&new) {
            return Some((new, state));
        }
        None
    }
}

pub struct MapDisplay<'a, T: Map>(pub &'a T);

impl<'a, T: Map> Display for MapDisplay<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let height = self.0.height();
        let width = self.0.width();
        for y in 0..height {
            for x in 0..width {
                write!(f, "{}", self.0.get(&(x, y)).unwrap())?;
            }
            if y != height - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

use crate::aoc::{read_lines, Answers, Position, Solution};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day21 {
    codes: Vec<String>,
    numpad_positions: [Position; 11],
}

impl Day21 {
    pub fn new() -> Day21 {
        Day21 {
            codes: Vec::new(),
            numpad_positions: [
                (1, 3),
                (0, 2),
                (1, 2),
                (2, 2),
                (0, 1),
                (1, 1),
                (2, 1),
                (0, 0),
                (1, 0),
                (2, 0),
                (2, 3),
            ],
        }
    }
}

impl Solution for Day21 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day21.txt";
        let lines = read_lines(filename)?;
        for line in lines.flatten() {
            self.codes.push(line);
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut memos = FxHashMap::default();
        let mut part1 = 0;
        let mut part2 = 0;
        for code in &self.codes {
            let num = code[0..code.len() - 1].parse::<usize>()?;
            part1 += num * self.seq_len(code, 2, &mut memos).ok_or("invalid code")?;
            part2 += num * self.seq_len(code, 25, &mut memos).ok_or("invalid code")?;
        }
        Ok(Answers::both(part1, part2))
    }
}

#[allow(dead_code)]
fn print_plan(code: &str, plan: &[DPad]) {
    println!(
        "{code}: {}",
        plan.iter().map(|b| format!("{b}")).collect::<String>()
    );
}

impl Day21 {
    fn seq_len(
        &self,
        code: &str,
        depth: usize,
        memos: &mut FxHashMap<(Vec<DPad>, usize), usize>,
    ) -> Option<usize> {
        Some(
            format!("A{code}")
                .chars()
                .map(|c| NumPad::parse(c))
                .collect::<Result<Vec<_>, _>>()
                .ok()?
                .iter()
                .tuple_windows()
                .map(|(start, end)| {
                    self.numpad_paths(*start, *end)
                        .iter()
                        .map(|path| {
                            if depth == 0 {
                                path.len()
                            } else {
                                self.dpad_seq_len(path, depth, memos)
                            }
                        })
                        .min()
                        .unwrap_or(0)
                })
                .sum(),
        )
    }

    fn dpad_seq_len(
        &self,
        path: &[DPad],
        depth: usize,
        memos: &mut FxHashMap<(Vec<DPad>, usize), usize>,
    ) -> usize {
        if depth == 0 {
            return path.len();
        }

        let key = (path.to_vec(), depth);

        if let Some(result) = memos.get(&key) {
            return *result;
        }

        let mut new = vec![DPad::BA];
        new.extend(path);

        let result = new
            .iter()
            .tuple_windows()
            .map(|(start, end)| {
                self.dpad_paths(*start, *end)
                    .iter()
                    .map(|path| self.dpad_seq_len(path, depth - 1, memos))
                    .min()
                    .unwrap_or(0)
            })
            .sum();
        memos.insert(key, result);
        result
    }

    fn numpad_paths(&self, start: NumPad, end: NumPad) -> Vec<Vec<DPad>> {
        let (sx, sy) = self.numpad_positions[start as usize];
        let (ex, ey) = self.numpad_positions[end as usize];
        let dx = ex as isize - sx as isize;
        let dy = ey as isize - sy as isize;
        let h_button = if dx > 0 { DPad::BR } else { DPad::BL };
        let v_button = if dy > 0 { DPad::BD } else { DPad::BU };
        if dx == 0 {
            let mut path = vec![v_button; dy.abs() as usize];
            path.push(DPad::BA);
            return vec![path];
        }
        if dy == 0 {
            let mut path = vec![h_button; dx.abs() as usize];
            path.push(DPad::BA);
            return vec![path];
        }

        match (sx, sy, ex, ey) {
            // Avoid missing space if moving from bottom to left...
            (_, 3, 0, _) => {
                let mut path = vec![v_button; dy.abs() as usize];
                path.extend(vec![h_button; dx.abs() as usize]);
                path.push(DPad::BA);
                vec![path]
            }
            // ...or left to bottom.
            (0, _, _, 3) => {
                let mut path = vec![h_button; dx.abs() as usize];
                path.extend(vec![v_button; dy.abs() as usize]);
                path.push(DPad::BA);
                vec![path]
            }
            _ => {
                let mut path1 = vec![v_button; dy.abs() as usize];
                path1.extend(vec![h_button; dx.abs() as usize]);
                path1.push(DPad::BA);
                let mut path2 = vec![h_button; dx.abs() as usize];
                path2.extend(vec![v_button; dy.abs() as usize]);
                path2.push(DPad::BA);
                vec![path1, path2]
            }
        }
    }

    fn dpad_paths(&self, start: DPad, end: DPad) -> Vec<Vec<DPad>> {
        match (start, end) {
            (DPad::BU, DPad::BD) => vec![vec![DPad::BD, DPad::BA]],
            (DPad::BU, DPad::BL) => vec![vec![DPad::BD, DPad::BL, DPad::BA]],
            (DPad::BU, DPad::BR) => vec![
                vec![DPad::BD, DPad::BR, DPad::BA],
                vec![DPad::BR, DPad::BD, DPad::BA],
            ],
            (DPad::BU, DPad::BA) => vec![vec![DPad::BR, DPad::BA]],
            (DPad::BD, DPad::BU) => vec![vec![DPad::BU, DPad::BA]],
            (DPad::BD, DPad::BL) => vec![vec![DPad::BL, DPad::BA]],
            (DPad::BD, DPad::BR) => vec![vec![DPad::BR, DPad::BA]],
            (DPad::BD, DPad::BA) => vec![
                vec![DPad::BU, DPad::BR, DPad::BA],
                vec![DPad::BR, DPad::BU, DPad::BA],
            ],
            (DPad::BL, DPad::BU) => vec![vec![DPad::BR, DPad::BU, DPad::BA]],
            (DPad::BL, DPad::BD) => vec![vec![DPad::BR, DPad::BA]],
            (DPad::BL, DPad::BR) => vec![vec![DPad::BR, DPad::BR, DPad::BA]],
            (DPad::BL, DPad::BA) => vec![vec![DPad::BR, DPad::BR, DPad::BU, DPad::BA]],
            (DPad::BR, DPad::BU) => vec![
                vec![DPad::BU, DPad::BL, DPad::BA],
                vec![DPad::BL, DPad::BU, DPad::BA],
            ],
            (DPad::BR, DPad::BD) => vec![vec![DPad::BL, DPad::BA]],
            (DPad::BR, DPad::BL) => vec![vec![DPad::BL, DPad::BL, DPad::BA]],
            (DPad::BR, DPad::BA) => vec![vec![DPad::BU, DPad::BA]],
            (DPad::BA, DPad::BU) => vec![vec![DPad::BL, DPad::BA]],
            (DPad::BA, DPad::BD) => vec![
                vec![DPad::BL, DPad::BD, DPad::BA],
                vec![DPad::BD, DPad::BL, DPad::BA],
            ],
            (DPad::BA, DPad::BL) => vec![vec![DPad::BD, DPad::BL, DPad::BL, DPad::BA]],
            (DPad::BA, DPad::BR) => vec![vec![DPad::BD, DPad::BA]],

            (a, b) if a == b => vec![vec![DPad::BA]],
            (_, _) => panic!("this is impossible by construction"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum NumPad {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    BA,
}

impl NumPad {
    fn parse(c: char) -> Result<Self, String> {
        match c {
            '0' => Ok(Self::B0),
            '1' => Ok(Self::B1),
            '2' => Ok(Self::B2),
            '3' => Ok(Self::B3),
            '4' => Ok(Self::B4),
            '5' => Ok(Self::B5),
            '6' => Ok(Self::B6),
            '7' => Ok(Self::B7),
            '8' => Ok(Self::B8),
            '9' => Ok(Self::B9),
            'A' => Ok(Self::BA),
            _ => Err("invalid char".into()),
        }
    }
}

impl Display for NumPad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::B0 => write!(f, "0"),
            Self::B1 => write!(f, "1"),
            Self::B2 => write!(f, "2"),
            Self::B3 => write!(f, "3"),
            Self::B4 => write!(f, "4"),
            Self::B5 => write!(f, "5"),
            Self::B6 => write!(f, "6"),
            Self::B7 => write!(f, "7"),
            Self::B8 => write!(f, "8"),
            Self::B9 => write!(f, "9"),
            Self::BA => write!(f, "A"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum DPad {
    BU,
    BD,
    BL,
    BR,
    BA,
}

impl Display for DPad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BU => write!(f, "^"),
            Self::BD => write!(f, "v"),
            Self::BL => write!(f, "<"),
            Self::BR => write!(f, ">"),
            Self::BA => write!(f, "A"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day21::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(248108usize, 303836969158972usize));
        Ok(())
    }
}

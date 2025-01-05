use crate::aoc::{read_chars, Answers, Solution};
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug)]
pub struct Day08 {
    antennas_by_freq: HashMap<char, Vec<Antenna>>,
    width: i32,
    height: i32,
}

impl Day08 {
    pub fn new() -> Day08 {
        Day08 {
            antennas_by_freq: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    fn check_pos(&self, pos: (i32, i32)) -> bool {
        if 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height {
            return true;
        }
        false
    }

    fn find_antinode(&self, pos1: (i32, i32), pos2: (i32, i32)) -> Option<(i32, i32)> {
        let from = sub(pos2, pos1);
        let antinode = add(pos2, from);
        if self.check_pos(antinode) {
            Some(antinode)
        } else {
            None
        }
    }

    fn find_harmonic_antinodes(&self, pos1: (i32, i32), pos2: (i32, i32)) -> Vec<(i32, i32)> {
        let from = sub(pos2, pos1);
        let mut antinode = pos2;
        let mut antinodes = Vec::new();
        while self.check_pos(antinode) {
            antinodes.push(antinode);
            antinode = add(antinode, from);
        }
        antinodes
    }
}

impl Solution for Day08 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let mut i = 0;
        let mut j = 0;
        let mut width = 0;
        for c in read_chars("./data/day08.txt")?.flatten() {
            match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                    if let Some(antennas) = self.antennas_by_freq.get_mut(&c) {
                        antennas.push(Antenna::new(c, (i, j)));
                    } else {
                        let antennas = vec![Antenna::new(c, (i, j))];
                        self.antennas_by_freq.insert(c, antennas);
                    }
                    i += 1;
                }
                '.' => {
                    i += 1;
                }
                '\n' => {
                    if width == 0 {
                        width = i;
                    }
                    i = 0;
                    j += 1;
                }
                _ => return Err(format!("invalid char {}", c).into()),
            }
        }

        self.width = width;
        self.height = j;

        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut antinodes = HashSet::new();
        let mut harmonic_antinodes = HashSet::new();
        for (_, antennas) in &self.antennas_by_freq {
            for (a, b) in pairs(antennas) {
                if let Some(antinode) = self.find_antinode(a.pos, b.pos) {
                    antinodes.insert(antinode);
                }
                for antinode in self.find_harmonic_antinodes(a.pos, b.pos) {
                    harmonic_antinodes.insert(antinode);
                }
            }
        }

        Ok(Answers::from(
            Some(antinodes.len()),
            Some(harmonic_antinodes.len()),
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Antenna {
    freq: char,
    pos: (i32, i32),
}

impl Antenna {
    fn new(freq: char, pos: (i32, i32)) -> Antenna {
        Antenna { freq, pos }
    }
}

fn pairs(antennas: &Vec<Antenna>) -> Vec<(Antenna, Antenna)> {
    let mut pairs = Vec::new();
    for antenna in antennas {
        for other in antennas {
            if antenna == other {
                continue;
            }
            pairs.push((*antenna, *other));
        }
    }
    pairs
}

fn add(pos1: (i32, i32), pos2: (i32, i32)) -> (i32, i32) {
    (pos1.0 + pos2.0, pos1.1 + pos2.1)
}

fn sub(pos1: (i32, i32), pos2: (i32, i32)) -> (i32, i32) {
    (pos1.0 - pos2.0, pos1.1 - pos2.1)
}

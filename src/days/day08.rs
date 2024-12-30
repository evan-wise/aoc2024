use crate::aoc::{read_chars, Answers, Solution};
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub struct Day08;

impl Solution for Day08 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve(&self) -> Result<Answers<Self::Part1, Self::Part2>, Box<dyn Error>> {
        let mut antennas_by_freq: HashMap<char, Vec<Antenna>> = HashMap::new();
        let mut i = 0;
        let mut j = 0;
        let mut width = 0;
        for c in read_chars("./data/day08.txt")?.flatten() {
            match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                    if let Some(antennas) = antennas_by_freq.get_mut(&c) {
                        antennas.push(Antenna::new(c, (i, j)));
                    } else {
                        let antennas = vec![Antenna::new(c, (i, j))];
                        antennas_by_freq.insert(c, antennas);
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

        let map = Map::new(width, j);
        let mut antinodes = HashSet::new();
        let mut harmonic_antinodes = HashSet::new();
        for (_, antennas) in antennas_by_freq {
            for (a, b) in pairs(antennas) {
                if let Some(antinode) = find_antinode(a.pos, b.pos, &map) {
                    antinodes.insert(antinode);
                }
                for antinode in find_harmonic_antinodes(a.pos, b.pos, &map) {
                    harmonic_antinodes.insert(antinode);
                }
            }
        }

        Answers::ok(Some(antinodes.len()), Some(harmonic_antinodes.len()))
    }
}

#[derive(Debug)]
struct Map {
    width: i32,
    height: i32,
}

impl Map {
    fn new(width: i32, height: i32) -> Map {
        Map { width, height }
    }

    fn check_pos(&self, pos: (i32, i32)) -> bool {
        if 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height {
            return true;
        }
        false
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

fn pairs(antennas: Vec<Antenna>) -> Vec<(Antenna, Antenna)> {
    let mut pairs = Vec::new();
    for antenna in &antennas {
        for other in &antennas {
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

fn find_antinode(pos1: (i32, i32), pos2: (i32, i32), map: &Map) -> Option<(i32, i32)> {
    let from = sub(pos2, pos1);
    let antinode = add(pos2, from);
    if map.check_pos(antinode) {
        Some(antinode)
    } else {
        None
    }
}

fn find_harmonic_antinodes(pos1: (i32, i32), pos2: (i32, i32), map: &Map) -> Vec<(i32, i32)> {
    let from = sub(pos2, pos1);
    let mut antinode = pos2;
    let mut antinodes = Vec::new();
    while map.check_pos(antinode) {
        antinodes.push(antinode);
        antinode = add(antinode, from);
    }
    antinodes
}

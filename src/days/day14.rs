use crate::aoc::{read_lines, Answers, Solution};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hash;

#[derive(Debug)]
pub struct Day14 {
    robots: Vec<Robot>,
}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 { robots: Vec::new() }
    }
}

impl Solution for Day14 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day14.txt")?;
        for line in lines.flatten() {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err("invalid line".into());
            }
            let pos = parse_position(parts[0])?;
            let vel = parse_velocity(parts[1])?;
            self.robots.push(Robot { pos, vel });
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let dim = (101, 103);
        let mut count_by_quadrant: HashMap<Quadrant, i32> = HashMap::new();
        for final_pos in self.robots.iter().map(|r| r.final_pos(100, dim)) {
            let quad = Quadrant::get(final_pos, dim);
            if let Some(count) = count_by_quadrant.get(&quad) {
                count_by_quadrant.insert(quad, count + 1);
            } else {
                count_by_quadrant.insert(quad, 1);
            }
        }
        let mut safety_score = 1;
        for (q, c) in &count_by_quadrant {
            safety_score *= match q {
                Quadrant::None => 1,
                _ => *c,
            }
        }
        let mut num_seconds = 1;
        loop {
            if no_dupes(self.robots.iter().map(|r| r.final_pos(num_seconds, dim))) {
                break;
            }
            num_seconds += 1;
        }
        Ok(Answers::from(Some(safety_score), Some(num_seconds)))
    }
}

fn parse_position(raw: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let parts = raw.split('=').collect::<Vec<&str>>();
    match parts[0] {
        "p" => {
            let subparts = parts[1].split(',').collect::<Vec<&str>>();
            if subparts.len() != 2 {
                return Err("invalid position".into());
            }
            Ok((subparts[0].parse::<i32>()?, subparts[1].parse::<i32>()?))
        }
        _ => Err("invalid position".into()),
    }
}

fn parse_velocity(raw: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let parts = raw.split('=').collect::<Vec<&str>>();
    match parts[0] {
        "v" => {
            let subparts = parts[1].split(',').collect::<Vec<&str>>();
            if subparts.len() != 2 {
                return Err("invalid velocity".into());
            }
            Ok((subparts[0].parse::<i32>()?, subparts[1].parse::<i32>()?))
        }
        _ => Err("invalid velocity".into()),
    }
}

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn final_pos(&self, t: i32, dim: (i32, i32)) -> (i32, i32) {
        let (w, h) = dim;
        let (px, py) = self.pos;
        let (vx, vy) = self.vel;
        ((px + vx * t).rem_euclid(w), (py + vy * t).rem_euclid(h))
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
    None,
}

impl Quadrant {
    fn get(pos: (i32, i32), dim: (i32, i32)) -> Quadrant {
        let (x, y) = pos;
        let (w, h) = dim;
        if x < w / 2 && y < h / 2 {
            Self::First
        } else if x > w / 2 && y < h / 2 {
            Self::Second
        } else if x < w / 2 && y > h / 2 {
            Self::Third
        } else if x > w / 2 && y > h / 2 {
            Self::Fourth
        } else {
            Self::None
        }
    }
}

fn no_dupes<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

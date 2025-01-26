use crate::aoc::{Answers, Solution};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Day24 {
    wires: FxHashMap<String, bool>,
    gates: VecDeque<Gate>,
}

impl Day24 {
    pub fn new() -> Day24 {
        Day24 {
            wires: FxHashMap::default(),
            gates: VecDeque::new(),
        }
    }
}

impl Solution for Day24 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day24.txt";
        let content = read_to_string(filename)?;
        let mut parts = content.split("\n\n");
        let raw_wires = parts.next().ok_or("missing wire states")?;
        let raw_gates = parts.next().ok_or("missing gate layout")?;
        let raw_gates = &raw_gates[..raw_gates.len() - 1];
        for line in raw_wires.split("\n") {
            let mut parts = line.split(": ");
            let label = parts.next().ok_or("missing wire label")?;
            let value = parts.next().ok_or("missing wire value")?;
            self.wires.insert(
                label.to_string(),
                match value {
                    "1" => Ok(true),
                    "0" => Ok(false),
                    _ => Err("invalid wire value"),
                }?,
            );
        }
        for line in raw_gates.split("\n") {
            let mut parts = line.split(" -> ");
            let operands = parts.next().ok_or("missing operands")?;
            let output = parts.next().ok_or("missing output")?.to_string();
            let mut parts = operands.split(" ");
            let left = parts.next().ok_or("missing left operand")?.to_string();
            let operator = parts.next().ok_or("missing operator")?;
            let right = parts.next().ok_or("missing right operand")?.to_string();
            let gate = match operator {
                "AND" => Ok(Gate::And(left, right, output)),
                "OR" => Ok(Gate::Or(left, right, output)),
                "XOR" => Ok(Gate::Xor(left, right, output)),
                _ => Err("invalid operator"),
            }?;
            self.gates.push_back(gate);
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        while let Some(gate) = self.gates.pop_front() {
            if !gate.eval(&mut self.wires) {
                self.gates.push_back(gate);
            }
        }
        let part1 = self.read_output();
        Ok(Answers::part1(part1))
    }
}

impl Day24 {
    fn read_output(&self) -> usize {
        let mut z_wires: Vec<String> = self
            .wires
            .keys()
            .filter(|k| k.starts_with("z"))
            .cloned()
            .collect();
        z_wires.sort();
        z_wires.iter().enumerate().fold(0, |a, (i, wire)| {
            a + if *self.wires.get(wire).unwrap_or(&false) {
                2usize.pow(i as u32)
            } else {
                0
            }
        })
    }
}

#[derive(Debug)]
enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

impl Gate {
    fn eval(&self, wires: &mut FxHashMap<String, bool>) -> bool {
        match self {
            Self::And(left, right, output) => {
                if let (Some(&l), Some(&r)) = (wires.get(left), wires.get(right)) {
                    wires.insert(output.clone(), l & r);
                    true
                } else {
                    false
                }
            }
            Self::Or(left, right, output) => {
                if let (Some(&l), Some(&r)) = (wires.get(left), wires.get(right)) {
                    wires.insert(output.clone(), l | r);
                    true
                } else {
                    false
                }
            }
            Self::Xor(left, right, output) => {
                if let (Some(&l), Some(&r)) = (wires.get(left), wires.get(right)) {
                    wires.insert(output.clone(), l ^ r);
                    true
                } else {
                    false
                }
            }
        }
    }
}

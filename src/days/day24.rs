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
            self.gates.push_back(match operator {
                "AND" => Ok(Gate {
                    left,
                    right,
                    output,
                    op: Op::And,
                }),
                "OR" => Ok(Gate {
                    left,
                    right,
                    output,
                    op: Op::Or,
                }),
                "XOR" => Ok(Gate {
                    left,
                    right,
                    output,
                    op: Op::Xor,
                }),
                _ => Err("invalid operator"),
            }?);
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let sorted_gates = self.simulate();
        let part1 = self.read('z');
        // We are assuming that no gates have been switched to "equivalent" positions, e.g.
        // swapping two carry outputs. This is in principle detectable by tracking which input gate
        // we started at but at least in my input there were no such swaps.
        let mut bad_gates: Vec<Gate> = Vec::new();
        for gate in &sorted_gates {
            let mut parents = sorted_gates
                .iter()
                .filter(|g| g.output == gate.left || g.output == gate.right)
                .map(|g| g.op.clone())
                .collect::<Vec<Op>>();
            parents.sort();
            let mut children = sorted_gates
                .iter()
                .filter(|g| g.left == gate.output || g.right == gate.output)
                .map(|g| g.op.clone())
                .collect::<Vec<Op>>();
            children.sort();
            match (
                gate.op.clone(),
                &parents[..],
                &children[..],
            ) {
                // input gates
                (Op::And, [], [Op::Or]) |
                (Op::Xor, [], [Op::And, Op::Xor]) |
                // output gates
                (Op::Xor, [Op::Or, Op::Xor], []) |
                // carry gates
                (Op::And, [Op::Or, Op::Xor], [Op::Or]) |
                (Op::Or, [Op::And, Op::And], [Op::And, Op::Xor]) |
                // special case for first carry gate
                (Op::And, [Op::And, Op::Xor], [Op::Or]) => (),
                // special cases for first input gates
                (Op::And, [], [Op::And, Op::Xor]) |
                (Op::Xor, [], []) if gate.left == "x00" || gate.right == "x00" => (),
                // special case for second output gate
                (Op::Xor, [Op::And, Op::Xor], []) if gate.output == "z01" => (),
                // special case for last output gate
                (Op::Or, [Op::And, Op::And], []) if gate.output == "z45" => (),
                // anything else is wrong
                _ => {
                    // if we already found the parent gate ignore the child
                    if let Some(_) = bad_gates.iter().find(|g| g.output == gate.left || g.output == gate.right) {
                        continue;
                    }
                    bad_gates.push(gate.clone());
                },
            }
        }
        let mut bad_outputs: Vec<String> =
            bad_gates.into_iter().map(|g| g.output.clone()).collect();
        bad_outputs.sort();
        let part2 = bad_outputs.join(",");
        Ok(Answers::both(part1, part2))
    }
}

impl Day24 {
    fn simulate(&mut self) -> Vec<Gate> {
        let mut sorted_gates = Vec::new();
        let mut gates = self.gates.clone();
        while let Some(gate) = gates.pop_front() {
            if !gate.eval(&mut self.wires) {
                gates.push_back(gate);
            } else {
                sorted_gates.push(gate)
            }
        }
        sorted_gates
    }

    fn read(&self, c: char) -> usize {
        let mut wires: Vec<String> = self
            .wires
            .keys()
            .filter(|k| k.chars().nth(0).unwrap() == c)
            .cloned()
            .collect();
        wires.sort();
        wires.iter().enumerate().fold(0, |a, (i, wire)| {
            a + if *self.wires.get(wire).unwrap_or(&false) {
                2usize.pow(i as u32)
            } else {
                0
            }
        })
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Gate {
    left: String,
    right: String,
    output: String,
    op: Op,
}

impl Gate {
    fn eval(&self, wires: &mut FxHashMap<String, bool>) -> bool {
        match self.op {
            Op::And => {
                if let (Some(&l), Some(&r)) = (wires.get(&self.left), wires.get(&self.right)) {
                    wires.insert(self.output.clone(), l & r);
                    true
                } else {
                    false
                }
            }
            Op::Or => {
                if let (Some(&l), Some(&r)) = (wires.get(&self.left), wires.get(&self.right)) {
                    wires.insert(self.output.clone(), l | r);
                    true
                } else {
                    false
                }
            }
            Op::Xor => {
                if let (Some(&l), Some(&r)) = (wires.get(&self.left), wires.get(&self.right)) {
                    wires.insert(self.output.clone(), l ^ r);
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day24::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(51745744348272usize, "bfq,bng,fjp,hkh,hmt,z18,z27,z31"));
        Ok(())
    }
}

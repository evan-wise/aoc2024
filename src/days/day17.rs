use crate::aoc::{read_lines, Solution, SolutionParts};
use std::error::Error;
use std::num::ParseIntError;
use std::path::Path;

pub struct Day17;

impl Solution for Day17 {
    fn solve(&self) -> Result<SolutionParts, Box<dyn Error>> {
        let mut computer = parse_input("./data/day17.txt")?;
        let output = computer.run()?;
        Ok((Some(output), None))
    }
}

fn parse_input<P: AsRef<Path>>(filename: P) -> Result<Computer, Box<dyn Error>> {
    let lines = read_lines(filename)?;
    let mut computer = Computer::new();
    for line in lines.flatten() {
        if line == "" {
            continue;
        }
        let parts = line.split(": ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("invalid line".into());
        }
        match parts[0] {
            "Register A" => computer.a = parts[1].parse::<u32>()?,
            "Register B" => computer.b = parts[1].parse::<u32>()?,
            "Register C" => computer.c = parts[1].parse::<u32>()?,
            "Program" => computer.instructions = parts[1].split(",").map(|i| i.parse::<u8>()).collect::<Result<Vec<u8>, ParseIntError>>()?,
            _ => {
                return Err("invalid line".into());
            }
        }
    }
    Ok(computer)
}

#[derive(Debug)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    instructions: Vec<u8>,
    instruction_pointer: usize,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            a: 0,
            b: 0,
            c: 0,
            instructions: Vec::new(),
            instruction_pointer: 0,
        }
    }

    fn run(&mut self) -> Result<String, String> {
        let mut output = Vec::new();
        let num_instructions = self.instructions.len();
        while self.instruction_pointer < num_instructions {
            let opcode = self.instructions[self.instruction_pointer];
            let operand = self.instructions[self.instruction_pointer+1];
            match opcode {
                0 => {
                    let divisor = (2 as u32).pow(self.combo(operand)?);
                    self.a /= divisor;
                }
                1 => {
                    self.b ^= operand as u32;
                }
                2 => {
                    self.b = self.combo(operand)? % 8;
                }
                3 => {
                    if self.a != 0 {
                        self.instruction_pointer = operand as usize;
                        continue;
                    } 
                }
                4 => {
                    self.b = self.b ^ self.c;
                }
                5 => {
                    output.push((self.combo(operand)? % 8).to_string());
                }
                6 => {
                    let divisor = (2 as u32).pow(self.combo(operand)?);
                    self.b = self.a / divisor;
                }
                7 => {
                    let divisor = (2 as u32).pow(self.combo(operand)?);
                    self.c = self.a / divisor;
                }
                _ => {
                    return Err(format!("invalid opcode {opcode}"));
                }
            }
            self.instruction_pointer += 2;
        }
        Ok(output.join(","))
    }

    fn combo(&self, operand: u8) -> Result<u32, String> {
        Ok(match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => {
                return Err(format!("invalid operand {operand}"));
            }
        })
    }
}


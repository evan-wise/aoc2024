use crate::aoc::{read_lines, Answers, Solution};
use std::error::Error;
use std::num::ParseIntError;

pub struct Day17 {
    computer: Computer,
}

impl Day17 {
    pub fn new() -> Day17 {
        Day17 {
            computer: Computer::new(),
        }
    }
}

impl Solution for Day17 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day17.txt")?;
        for line in lines.flatten() {
            if line == "" {
                continue;
            }
            let parts = line.split(": ").collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err("invalid line".into());
            }
            match parts[0] {
                "Register A" => self.computer.a = parts[1].parse::<u128>()?,
                "Register B" => self.computer.b = parts[1].parse::<u128>()?,
                "Register C" => self.computer.c = parts[1].parse::<u128>()?,
                "Program" => {
                    self.computer.instructions = parts[1]
                        .split(",")
                        .map(|i| i.parse::<u8>())
                        .collect::<Result<Vec<u8>, ParseIntError>>()?
                }
                _ => {
                    return Err("invalid line".into());
                }
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let output = self.computer.run()?;
        let output_str = output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let quine_a = backtrack(&self.computer)?;
        Ok(Answers::from(Some(output_str), Some(quine_a)))
    }
}

// Explanation:
// This may not work for a general program but:
//   1) From the construction of the problem, we know that the program halts and thus that register
//      A is 0 after the final iteration.
//   2) From inspection, the input programs always end with a jump to 0.
//   3) From inspection, the state of registers B and C depends only on the current value of
//      register A, so we don't have to bother tracking this between iterations.
// Register A must encode the instructions as outputs in some way, thus we must consume 3 bits of
// register A during each iteration of the loop. If we know the final value of A after the
// computation, we can try every 3 bit suffix for that value until we find the one that outputs the
// previous value in the instructions.
fn backtrack(computer: &Computer) -> Result<u128, String> {
    let mut no_jump = computer.clone();
    no_jump.instructions.pop();
    no_jump.instructions.pop();
    let mut stack = vec![(0, computer.instructions.len() - 1)];
    let mut final_a = 0;
    while let Some((a, i)) = stack.pop() {
        for suffix in 0..8 {
            let test_a = (a << 3) | suffix;
            no_jump.a = test_a;
            no_jump.instruction_pointer = 0;
            let output = no_jump.run()?[0];
            if output == computer.instructions[i] {
                if i == 0 {
                    final_a = test_a;
                } else {
                    stack.push((test_a, i - 1));
                }
            }
        }
    }
    Ok(final_a)
}

#[derive(Clone, Debug)]
struct Computer {
    a: u128,
    b: u128,
    c: u128,
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

    fn run(&mut self) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let num_instructions = self.instructions.len();
        while self.instruction_pointer < num_instructions {
            let opcode = self.instructions[self.instruction_pointer];
            let operand = self.instructions[self.instruction_pointer + 1];
            match opcode {
                0 => {
                    self.a >>= self.combo(operand)?;
                }
                1 => {
                    self.b ^= operand as u128;
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
                    output.push((self.combo(operand)? % 8) as u8);
                }
                6 => {
                    self.b = self.a >> self.combo(operand)?;
                }
                7 => {
                    self.c = self.a >> self.combo(operand)?;
                }
                _ => {
                    return Err(format!("invalid opcode {opcode}"));
                }
            }
            self.instruction_pointer += 2;
        }
        Ok(output)
    }

    fn combo(&self, operand: u8) -> Result<u128, String> {
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

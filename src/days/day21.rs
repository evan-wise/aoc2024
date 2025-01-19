use crate::aoc::{read_lines, Answers, Position, Solution};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Day21 {
    codes: Vec<String>,
    numpad_positions: [Position; 11],
    dpad_positions: [Position; 5],
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
            dpad_positions: [(1, 0), (1, 1), (0, 1), (2, 1), (2, 0)],
        }
    }
}

impl Solution for Day21 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./examples/day21.txt";
        let lines = read_lines(filename)?;
        for line in lines.flatten() {
            self.codes.push(line);
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut part1 = 0usize;
        for code in &self.codes {
            let plan1 = self.plan_numpad_code(code).ok_or("invalid code")?;
            let plan2 = self.plan_dpad_code(&plan1);
            let plan3 = self.plan_dpad_code(&plan2);
            let num = code[0..code.len()-1].parse::<usize>()?;
            part1 += num * plan3.len();
        }
        Ok(Answers::part1(part1))
    }
}

#[allow(dead_code)]
fn print_plan(code: &str, plan: &[DPad]) {
    println!("{code}: {}", plan.iter().map(|b| format!("{b}")).collect::<String>());
}

impl Day21 {
    fn plan_numpad_code(&self, code: &str) -> Option<Vec<DPad>> {
        let mut plan = Vec::new();
        let mut curr_button = NumPad::BA;
        let (mut cx, mut cy) = self.numpad_positions[curr_button as usize];
        for c in code.chars() {
            let next_button = NumPad::parse(c).ok()?;
            let (nx, ny) = self.numpad_positions[next_button as usize];
            match curr_button {
                NumPad::BA | NumPad::B0 => {
                    while cy > ny {
                        plan.push(DPad::BU);
                        cy -= 1;
                    }
                    while cx > nx {
                        plan.push(DPad::BL);
                        cx -= 1;
                    }
                    while cx < nx {
                        plan.push(DPad::BR);
                        cx += 1;
                    }
                    plan.push(DPad::BA);
                }
                _ => {
                    while cx > nx {
                        plan.push(DPad::BL);
                        cx -= 1;
                    }
                    while cx < nx {
                        plan.push(DPad::BR);
                        cx += 1;
                    }
                    while cy > ny {
                        plan.push(DPad::BU);
                        cy -= 1;
                    }
                    while cy < ny {
                        plan.push(DPad::BD);
                        cy += 1;
                    }
                    plan.push(DPad::BA);
                }
            }
            curr_button = next_button;
        }
        Some(plan)
    }

    fn plan_dpad_code(&self, code: &[DPad]) -> Vec<DPad> {
        let mut plan = Vec::new();
        let mut curr_button = DPad::BA;
        let (mut cx, mut cy) = self.dpad_positions[curr_button as usize];
        for &next_button in code {
            let (nx, ny) = self.dpad_positions[next_button as usize];
            match curr_button {
                DPad::BA | DPad::BU => {
                    while cy < ny {
                        plan.push(DPad::BD);
                        cy += 1;
                    }
                    while cx > nx {
                        plan.push(DPad::BL);
                        cx -= 1;
                    }
                    while cx < nx {
                        plan.push(DPad::BR);
                        cx += 1;
                    }
                    plan.push(DPad::BA);
                }
                _ => {
                    while cx > nx {
                        plan.push(DPad::BL);
                        cx -= 1;
                    }
                    while cx < nx {
                        plan.push(DPad::BR);
                        cx += 1;
                    }
                    while cy > ny {
                        plan.push(DPad::BU);
                        cy -= 1;
                    }
                    plan.push(DPad::BA);
                }
            }
            curr_button = next_button
        }
        plan
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

#[derive(Clone, Copy, Debug)]
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

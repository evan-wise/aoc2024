use std::error::Error;

use crate::days::Solution;
use crate::aoc::read_lines;

pub struct Day13;

impl Solution for Day13 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        let mut claw_machines = Vec::new();
        let mut maybe_button_a: Option<Button> = None;
        let mut maybe_button_b: Option<Button> = None;
        let mut maybe_prize: Option<Prize> = None;
        let lines = read_lines("./examples/day13.txt")?;
        for line in lines.flatten() {
            if line == "" {
                match (maybe_button_a, maybe_button_b, maybe_prize) {
                    (Some(button_a), Some(button_b), Some(prize)) => {
                        claw_machines.push(ClawMachine { button_a, button_b, prize });
                    }
                    _ => {
                        return Err("invalid claw machine".into());
                    }
                }
                continue;
            }

            let parts = line.split(": ").collect::<Vec<&str>>();
            match parts[0] {
                "Button A" => {
                    maybe_button_a = Some(Button::from(parts[1])?);
                }
                "Button B" => {
                    maybe_button_b = Some(Button::from(parts[1])?);
                }
                "Prize" => {
                    maybe_prize = Some(Prize::from(parts[1])?);
                }
                _ => {
                    return Err("invalid line".into());
                }
            }
        }
        println!("{:?}", claw_machines);
        Ok(())
    }
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

#[derive(Clone, Copy, Debug)]
struct Button {
    x: i32,
    y: i32,
}

impl Button {
    fn from(raw: &str) -> Result<Button, Box<dyn Error>> {
        let parts = raw.split(", ");
        let mut maybe_x: Option<i32> = None;
        let mut maybe_y: Option<i32> = None;
        for part in parts {
            let subparts = part.split("+").collect::<Vec<&str>>();
            match subparts[0] {
                "X" => {
                    maybe_x = Some(subparts[1].parse::<i32>()?);
                }
                "Y" => {
                    maybe_y = Some(subparts[1].parse::<i32>()?);
                }
                _ => {
                    return Err("invalid button".into());
                }
            }
        }
        match (maybe_x, maybe_y) {
            (Some(x), Some(y)) => Ok(Button { x, y }),
            _ => Err("invalid button".into())
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Prize {
    x: i32, 
    y: i32,
}

impl Prize {
    fn from(raw: &str) -> Result<Prize, Box<dyn Error>> {
        let parts = raw.split(", ");
        let mut maybe_x: Option<i32> = None;
        let mut maybe_y: Option<i32> = None;
        for part in parts {
            let subparts = part.split("=").collect::<Vec<&str>>();
            match subparts[0] {
                "X" => {
                    maybe_x = Some(subparts[1].parse::<i32>()?);
                }
                "Y" => {
                    maybe_y = Some(subparts[1].parse::<i32>()?);
                }
                _ => {
                    return Err("invalid prize".into());
                }
            }
        }
        match (maybe_x, maybe_y) {
            (Some(x), Some(y)) => Ok(Prize { x, y }),
            _ => Err("invalid prize".into())
        }
    }
}

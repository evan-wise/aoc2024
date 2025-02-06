use crate::aoc::{read_lines, Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day13 {
    claw_machines: Vec<ClawMachine>,
}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 {
            claw_machines: Vec::new(),
        }
    }
}

impl Solution for Day13 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day13.txt")?;
        let mut maybe_button_a: Option<Button> = None;
        let mut maybe_button_b: Option<Button> = None;
        let mut maybe_prize: Option<Prize> = None;
        for line in lines.flatten() {
            if line == "" {
                match (maybe_button_a, maybe_button_b, maybe_prize) {
                    (Some(button_a), Some(button_b), Some(prize)) => {
                        self.claw_machines.push(ClawMachine {
                            button_a,
                            button_b,
                            prize,
                        });
                        maybe_button_a = None;
                        maybe_button_b = None;
                        maybe_prize = None;
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
                    maybe_button_a = Some(Button::parse(parts[1])?);
                }
                "Button B" => {
                    maybe_button_b = Some(Button::parse(parts[1])?);
                }
                "Prize" => {
                    maybe_prize = Some(Prize::parse(parts[1])?);
                }
                _ => {
                    return Err("invalid line".into());
                }
            }
        }
        match (maybe_button_a, maybe_button_b, maybe_prize) {
            (Some(button_a), Some(button_b), Some(prize)) => {
                self.claw_machines.push(ClawMachine {
                    button_a,
                    button_b,
                    prize,
                });
            }
            _ => {
                return Err("invalid claw machine".into());
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let cost1 = compute_cost(&self.claw_machines);
        rescale_prizes(&mut self.claw_machines);
        let cost2 = compute_cost(&self.claw_machines);
        Ok(Answers::both(cost1, cost2))
    }
}

fn compute_cost(claw_machines: &Vec<ClawMachine>) -> i64 {
    let mut cost = 0;
    for claw_machine in claw_machines {
        match solve_system(
            claw_machine.button_a.x,
            claw_machine.button_b.x,
            claw_machine.button_a.y,
            claw_machine.button_b.y,
            claw_machine.prize.x,
            claw_machine.prize.y,
        ) {
            DiophantineSolution::Unique(a, b) => {
                cost += 3 * a + b;
            }
            // Really we should handle this but it turns out there are none in the input
            DiophantineSolution::Parametric(_, _, _, _) => (),
            DiophantineSolution::None => (),
        }
    }
    cost
}

fn rescale_prizes(claw_machines: &mut Vec<ClawMachine>) {
    for claw_machine in claw_machines {
        claw_machine.prize.x += 10000000000000;
        claw_machine.prize.y += 10000000000000;
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
    x: i64,
    y: i64,
}

impl Button {
    fn parse(raw: &str) -> Result<Button, Box<dyn Error>> {
        let parts = raw.split(", ");
        let mut maybe_x: Option<i64> = None;
        let mut maybe_y: Option<i64> = None;
        for part in parts {
            let subparts = part.split("+").collect::<Vec<&str>>();
            match subparts[0] {
                "X" => {
                    maybe_x = Some(subparts[1].parse::<i64>()?);
                }
                "Y" => {
                    maybe_y = Some(subparts[1].parse::<i64>()?);
                }
                _ => {
                    return Err("invalid button".into());
                }
            }
        }
        match (maybe_x, maybe_y) {
            (Some(x), Some(y)) => Ok(Button { x, y }),
            _ => Err("invalid button".into()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Prize {
    x: i64,
    y: i64,
}

impl Prize {
    fn parse(raw: &str) -> Result<Prize, Box<dyn Error>> {
        let parts = raw.split(", ");
        let mut maybe_x: Option<i64> = None;
        let mut maybe_y: Option<i64> = None;
        for part in parts {
            let subparts = part.split("=").collect::<Vec<&str>>();
            match subparts[0] {
                "X" => {
                    maybe_x = Some(subparts[1].parse::<i64>()?);
                }
                "Y" => {
                    maybe_y = Some(subparts[1].parse::<i64>()?);
                }
                _ => {
                    return Err("invalid prize".into());
                }
            }
        }
        match (maybe_x, maybe_y) {
            (Some(x), Some(y)) => Ok(Prize { x, y }),
            _ => Err("invalid prize".into()),
        }
    }
}

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x1, y1) = gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;

    (g, x, y)
}

#[derive(Debug, PartialEq)]
enum DiophantineSolution {
    /// Single unique solution (x, y)
    Unique(i64, i64),
    /// Parametric solution (x0, y0, t, u) representing:
    /// x = x0 + t * n
    /// y = y0 + u * n
    /// where n is any integer
    Parametric(i64, i64, i64, i64),
    /// No integer solutions exist
    None,
}

fn solve_diophantine_eq(a: i64, b: i64, c: i64) -> DiophantineSolution {
    if a == 0 && b == 0 {
        if c == 0 {
            return DiophantineSolution::Parametric(0, 0, 1, 1);
        } else {
            return DiophantineSolution::None;
        }
    }

    if a == 0 {
        if c % b == 0 {
            return DiophantineSolution::Parametric(0, c / b, 1, 1);
        } else {
            return DiophantineSolution::None;
        }
    }

    if b == 0 {
        if c % a == 0 {
            return DiophantineSolution::Parametric(c / a, 0, 1, 1);
        } else {
            return DiophantineSolution::None;
        }
    }

    let (g, mut x0, mut y0) = gcd(a.abs(), b.abs());

    if c % g != 0 {
        return DiophantineSolution::None;
    }

    if a < 0 {
        x0 = -x0;
    }

    if b < 0 {
        y0 = -y0;
    }

    x0 = x0 * (c / g);
    y0 = y0 * (c / g);

    DiophantineSolution::Parametric(x0, y0, b / g, -a / g)
}

fn solve_system(a1: i64, b1: i64, a2: i64, b2: i64, c1: i64, c2: i64) -> DiophantineSolution {
    let det = a1 * b2 - b1 * a2;
    if det == 0 {
        return handle_det_zero(a1, b1, a2, b2, c1, c2);
    }

    // Cramer's rule
    let x_num = c1 * b2 - b1 * c2;
    let y_num = a1 * c2 - c1 * a2;
    if x_num % det != 0 || y_num % det != 0 {
        return DiophantineSolution::None;
    }
    DiophantineSolution::Unique(x_num / det, y_num / det)
}

fn handle_det_zero(a1: i64, b1: i64, a2: i64, b2: i64, c1: i64, c2: i64) -> DiophantineSolution {
    // All zeros case
    if a1 == 0 && a2 == 0 && b1 == 0 && b2 == 0 {
        return if c1 == 0 && c2 == 0 {
            DiophantineSolution::Parametric(0, 0, 1, 1)
        } else {
            DiophantineSolution::None
        };
    }

    // Only y
    if a1 == 0 && a2 == 0 {
        if b1 != 0 {
            return if b1 * c2 == b2 * c1 && c1 % b1 == 0 {
                DiophantineSolution::Parametric(0, 0, 1, c1 / b1)
            } else {
                DiophantineSolution::None
            };
        }

        if b2 != 0 {
            return if b1 * c2 == b2 * c1 && c2 % b2 == 0 {
                DiophantineSolution::Parametric(0, 0, 1, c2 / b2)
            } else {
                DiophantineSolution::None
            };
        }
    }

    // Only x
    if b1 == 0 && b2 == 0 {
        if a1 != 0 {
            return if a1 * c2 == a2 * c1 && c1 % a1 == 0 {
                DiophantineSolution::Parametric(0, 0, c1 / a1, 1)
            } else {
                DiophantineSolution::None
            };
        }

        if a2 != 0 {
            return if a1 * c2 == a2 * c1 && c2 % a2 == 0 {
                DiophantineSolution::Parametric(0, 0, c2 / a2, 1)
            } else {
                DiophantineSolution::None
            };
        }
    }

    // Check if equations are multiples of each other.
    if a1 * b2 != a2 * b1 || a1 * c2 != a2 * c1 {
        return DiophantineSolution::None;
    }
    return solve_diophantine_eq(a1, b1, c1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day13::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(29187i64, 99968222587852i64));
        Ok(())
    }
}

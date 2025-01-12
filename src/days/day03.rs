use crate::aoc::{read_chars, Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day03 {
    chars: Vec<char>,
}

impl Day03 {
    pub fn new() -> Day03 {
        Day03 { chars: Vec::new() }
    }
}

impl Solution for Day03 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        self.chars.extend(read_chars("./data/day03.txt")?.flatten());
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        Ok(Answers::both(
            compute_total(&self.chars, false)?,
            compute_total(&self.chars, true)?,
        ))
    }
}

enum State {
    Disabled,
    Seeking,
    MulFound,
    OpenParenFound,
    FirstNumFound,
    CommaFound,
    SecondNumFound,
    CloseParenFound,
}

fn compute_total(chars: &Vec<char>, handle_dos: bool) -> Result<i32, Box<dyn Error>> {
    let mut state = State::Seeking;
    let mut temp = String::new();
    let mut first_num = 0;
    let mut second_num;
    let mut total = 0;
    for char in chars {
        match state {
            State::Disabled => {
                temp.push(*char);
                if temp.len() > 4 {
                    temp = temp[1..].to_string();
                }
                if &temp == "do()" {
                    state = State::Seeking;
                    temp = String::new();
                }
            }
            State::Seeking => {
                temp.push(*char);
                if temp.len() > 7 {
                    temp = temp[1..].to_string();
                }
                if temp.ends_with("mul") {
                    state = State::MulFound;
                    temp = String::new();
                } else if &temp == "don't()" && handle_dos {
                    state = State::Disabled;
                    temp = String::new();
                }
            }
            State::MulFound => {
                if *char == '(' {
                    state = State::OpenParenFound;
                } else {
                    state = State::Seeking;
                    temp = char.to_string();
                }
            }
            State::OpenParenFound => {
                if char.is_digit(10) {
                    state = State::FirstNumFound;
                    temp.push(*char);
                } else {
                    state = State::Seeking;
                    temp = char.to_string();
                }
            }
            State::FirstNumFound => {
                if char.is_digit(10) {
                    temp.push(*char);
                    if temp.len() > 3 {
                        state = State::Seeking;
                        temp = String::new();
                    }
                } else if *char == ',' {
                    state = State::CommaFound;
                    first_num = temp.parse::<i32>()?;
                    temp = String::new();
                } else {
                    state = State::Seeking;
                    temp = char.to_string();
                }
            }
            State::CommaFound => {
                if char.is_digit(10) {
                    state = State::SecondNumFound;
                    temp.push(*char);
                } else {
                    state = State::Seeking;
                    temp = char.to_string();
                }
            }
            State::SecondNumFound => {
                if char.is_digit(10) {
                    temp.push(*char);
                    if temp.len() > 3 {
                        state = State::Seeking;
                        temp = String::new();
                    }
                } else if *char == ')' {
                    state = State::CloseParenFound;
                    second_num = temp.parse::<i32>()?;
                    total += first_num * second_num;
                    temp = String::new();
                } else {
                    state = State::Seeking;
                    temp = char.to_string();
                }
            }
            State::CloseParenFound => {
                state = State::Seeking;
                temp = char.to_string();
            }
        }
    }
    Ok(total)
}

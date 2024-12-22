use crate::aoc::{read_chars, Solution};
use std::error::Error;

pub struct Day03;

impl Solution for Day03 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        println!(
            "The sum total of all the multiplications is: {}",
            compute_total(false)?
        );
        println!(
            "The sum total of all the multiplications if conditionals are handled is: {}",
            compute_total(true)?
        );
        Ok(())
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

fn compute_total(handle_dos: bool) -> Result<i32, Box<dyn Error>> {
    let chars = read_chars("./data/day03.txt")?;
    let mut state = State::Seeking;
    let mut temp = String::new();
    let mut first_num = 0;
    let mut second_num = 0;
    let mut total = 0;
    for char in chars.flatten() {
        match state {
            State::Disabled => {
                temp.push(char);
                if temp.len() > 4 {
                    temp = temp[1..].to_string();
                }
                if &temp == "do()" {
                    state = State::Seeking;
                    temp = String::new();
                }
            }
            State::Seeking => {
                temp.push(char);
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
                if char == '(' {
                    state = State::OpenParenFound;
                } else {
                    state = State::Seeking;
                    temp = char.to_string();
                }
            }
            State::OpenParenFound => {
                if char.is_digit(10) {
                    state = State::FirstNumFound;
                    temp.push(char);
                } else {
                    state = State::Seeking;
                    temp = char.to_string();
                }
            }
            State::FirstNumFound => {
                if char.is_digit(10) {
                    temp.push(char);
                    if temp.len() > 3 {
                        state = State::Seeking;
                        temp = String::new();
                    }
                } else if char == ',' {
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
                    temp.push(char);
                } else {
                    state = State::Seeking;
                    temp = char.to_string();
                }
            }
            State::SecondNumFound => {
                if char.is_digit(10) {
                    temp.push(char);
                    if temp.len() > 3 {
                        state = State::Seeking;
                        temp = String::new();
                    }
                } else if char == ')' {
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

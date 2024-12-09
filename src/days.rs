pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

use std::error::Error;

pub trait Solution {
    fn solve(&self) -> Result<(), Box<dyn Error>>;
}

pub fn solutions() -> Vec<Box<dyn Solution>> {
    vec![
        Box::new(day01::Day01 {}),
        Box::new(day02::Day02 {}),
        Box::new(day03::Day03 {}),
        Box::new(day04::Day04 {}),
        Box::new(day05::Day05 {}),
        Box::new(day06::Day06 {}),
        Box::new(day07::Day07 {}),
    ]
}


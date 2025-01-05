pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;

pub fn solutions() -> Vec<Box<dyn crate::aoc::Solution>> {
    vec![
        Box::new(day01::Day01::new()),
        Box::new(day02::Day02::new()),
        Box::new(day03::Day03::new()),
        Box::new(day04::Day04::new()),
        Box::new(day05::Day05::new()),
        Box::new(day06::Day06::new()),
        Box::new(day07::Day07::new()),
        Box::new(day08::Day08::new()),
        Box::new(day09::Day09::new()),
        Box::new(day10::Day10::new()),
        Box::new(day11::Day11::new()),
        Box::new(day12::Day12::new()),
        Box::new(day13::Day13::new()),
        Box::new(day14::Day14::new()),
        Box::new(day15::Day15::new()),
        Box::new(day16::Day16::new()),
        Box::new(day17::Day17::new()),
        Box::new(day18::Day18::new()),
        Box::new(day19::Day19::new()),
    ]
}

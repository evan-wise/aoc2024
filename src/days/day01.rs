use crate::aoc::{read_lines, Answers, Solution};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct Day01 {
    list1: Vec<i32>,
    list2: Vec<i32>,
}

impl Day01 {
    pub fn new() -> Self {
        Day01 {
            list1: vec![0; 1000],
            list2: vec![0; 1000],
        }
    }
}

impl Solution for Day01 {
    type Part1 = i32;
    type Part2 = i32;

    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day01.txt")?;
        for (i, line) in lines.flatten().enumerate() {
            let mut iter = line.split("   ");
            let str1 = iter.next().ok_or("Malformed line")?;
            let num1 = str1.parse::<i32>()?;
            let str2 = iter.next().ok_or("Malformed line")?;
            let num2 = str2.parse::<i32>()?;
            self.list1[i] = num1;
            self.list2[i] = num2;
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers<Self::Part1, Self::Part2>, Box<dyn Error>> {
        if let Ok(lines) = read_lines("./data/day01.txt") {
            for (i, line) in lines.flatten().enumerate() {
                let mut iter = line.split("   ");
                let str1 = iter.next().ok_or("Malformed line")?;
                let num1 = str1.parse::<i32>()?;
                let str2 = iter.next().ok_or("Malformed line")?;
                let num2 = str2.parse::<i32>()?;
                self.list1[i] = num1;
                self.list2[i] = num2;
            }
        }
        self.list1.sort();
        self.list2.sort();

        let mut total_distance = 0;
        for (l, r) in self.list1.iter().zip(self.list2.iter()) {
            total_distance += (l - r).abs();
        }

        let mut cum_score = 0;
        let mut score_by_id = HashMap::new();
        for location_id in &self.list1 {
            if let Some(score) = score_by_id.get(&location_id) {
                cum_score += score;
            } else {
                let mut count = 0;
                for other_id in &self.list2 {
                    if other_id == location_id {
                        count += 1;
                    }
                }
                let score = location_id * count;
                score_by_id.insert(location_id, score);
                cum_score += score;
            }
        }

        Answers::ok(Some(total_distance), Some(cum_score))
    }
}

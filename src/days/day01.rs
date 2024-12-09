use crate::aoc::read_lines;
use crate::days::Solution;
use std::collections::HashMap;
use std::error::Error;

pub struct Day01;

impl Solution for Day01 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        let mut list1 = vec![0; 1000];
        let mut list2 = vec![0; 1000];
        if let Ok(lines) = read_lines("./data/day01.txt") {
            for (i, line) in lines.flatten().enumerate() {
                let mut iter = line.split("   ");
                let str1 = iter.next().ok_or("Malformed line")?;
                let num1 = str1.parse::<i32>()?;
                let str2 = iter.next().ok_or("Malformed line")?;
                let num2 = str2.parse::<i32>()?;
                list1[i] = num1;
                list2[i] = num2;
            }
        }
        list1.sort();
        list2.sort();

        let mut sum = 0;
        for (l, r) in list1.iter().zip(list2.iter()) {
            sum += (l - r).abs();
        }
        println!("The total distance is: {}", sum);

        let mut cum_score = 0;
        let mut score_by_id = HashMap::new();
        for location_id in list1.iter() {
            if let Some(score) = score_by_id.get(&location_id) {
                cum_score += score;
            } else {
                let mut count = 0;
                for other_id in list2.iter() {
                    if other_id == location_id {
                        count += 1;
                    }
                }
                let score = location_id * count;
                score_by_id.insert(location_id, score);
                cum_score += score;
            }
        }
        println!("The similarity score is: {}", cum_score);

        Ok(())
    }
}

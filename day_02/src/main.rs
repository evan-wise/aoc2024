use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;
use std::io::{self, BufRead};
use std::error::Error;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut nums = Vec::new();
    for raw in line.split(" ") {
        let num = raw.parse::<i32>()?;
        nums.push(num);
    }
    Ok(nums)
}

fn check_safety(nums: &Vec<i32>) -> bool {
    let mut is_safe = true;
    let mut maybe_prev = None;
    let mut maybe_dir = None;
    let mut nums_iter = nums.iter();
    while let Some(num) = nums_iter.next() {
        if let Some(prev) = maybe_prev {
            let diff: i32 = num - prev;
            if diff.abs() < 1 || diff.abs() > 3 {
                is_safe = false;
                break;
            }
            if let Some(dir) = maybe_dir {
                if dir != diff.signum() {
                    is_safe = false;
                    break;
                }
            } else {
                maybe_dir = Some(diff.signum())
            }
        }
        maybe_prev = Some(num);

    }
    is_safe
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    if let Ok(lines) = read_lines("./data/reports.txt") {
        for line in lines.flatten() {
            let nums = parse_line(&line)?;
            if check_safety(&nums) {
                count += 1;
            }
        }
    }
    println!("There are {} safe reports.", count);
    Ok(())
}

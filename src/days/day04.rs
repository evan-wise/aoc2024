use crate::aoc::{read_lines, Answers, Solution};
use std::error::Error;

#[derive(Debug)]
pub struct Day04 {
    wordsearch: Vec<Vec<u8>>,
}

impl Day04 {
    pub fn new() -> Day04 {
        Day04 {
            wordsearch: Vec::new(),
        }
    }
}

impl Solution for Day04 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let lines = read_lines("./data/day04.txt")?;
        self.wordsearch
            .extend(lines.flatten().map(|s| s.into_bytes()));
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let mut xmas_total = 0;
        let mut cross_total = 0;
        for (i, s) in self.wordsearch.iter().enumerate() {
            for (j, _c) in s.iter().enumerate() {
                xmas_total += count_xmas(i, j, &self.wordsearch);
                if check_cross(i, j, &self.wordsearch) {
                    cross_total += 1;
                }
            }
        }
        Ok(Answers::both(xmas_total, cross_total))
    }
}

fn count_xmas(i: usize, j: usize, wordsearch: &Vec<Vec<u8>>) -> i32 {
    let mut count = 0;
    let word = "XMAS".as_bytes();
    let x_len = wordsearch[i].len();
    let y_len = wordsearch.len();

    if wordsearch[i][j] != word[0] {
        return count;
    }

    // Look in each direction.
    for k in 1..=3 {
        if j + k >= x_len {
            break;
        }

        if wordsearch[i][j + k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if j + k >= x_len || i + k >= y_len {
            break;
        }

        if wordsearch[i + k][j + k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if i + k >= y_len {
            break;
        }

        if wordsearch[i + k][j] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if j < k || i + k >= y_len {
            break;
        }

        if wordsearch[i + k][j - k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if j < k {
            break;
        }

        if wordsearch[i][j - k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if j < k || i < k {
            break;
        }

        if wordsearch[i - k][j - k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if i < k {
            break;
        }

        if wordsearch[i - k][j] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if j + k >= x_len || i < k {
            break;
        }

        if wordsearch[i - k][j + k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    count
}

fn check_cross(i: usize, j: usize, wordsearch: &Vec<Vec<u8>>) -> bool {
    let x_len = wordsearch[i].len();
    let y_len = wordsearch.len();
    let mut cross = String::new();
    cross.push(wordsearch[i][j] as char);

    if j + 2 < x_len {
        cross.push(wordsearch[i][j + 2] as char);
    }

    if j + 1 < x_len && i + 1 < y_len {
        cross.push(wordsearch[i + 1][j + 1] as char);
    }

    if i + 2 < y_len {
        cross.push(wordsearch[i + 2][j] as char);
    }

    if j + 2 < x_len && i + 2 < y_len {
        cross.push(wordsearch[i + 2][j + 2] as char);
    }

    match cross.as_str() {
        "MMASS" | "SMASM" | "SSAMM" | "MSAMS" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution() -> Result<(), Box<dyn Error>> {
        let mut solution = Day04::new();
        solution.parse_input()?;
        let answers = solution.solve()?;
        assert_eq!(answers, Answers::both(2414, 1871));
        Ok(())
    }
}

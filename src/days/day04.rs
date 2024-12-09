use crate::aoc::read_lines;
use crate::days::Solution;
use std::error::Error;

pub struct Day04;

impl Solution for Day04 {
    fn solve(&self) -> Result<(), Box<dyn Error>> {
        if let Ok(lines) = read_lines("./data/wordsearch.txt") {
            let wordsearch: Vec<Vec<u8>> = lines.flatten().map(|s| s.into_bytes()).collect();
            let mut xmas_total = 0;
            let mut cross_total = 0;
            for (i, s) in wordsearch.iter().enumerate() {
                for (j, _c) in s.iter().enumerate() {
                    xmas_total += count_xmas(i, j, &wordsearch);
                    if check_cross(i, j, &wordsearch) {
                        cross_total += 1;
                    }
                }
            }
            println!("The word \"XMAS\" appears {} time(s).", xmas_total);
            println!("The \"X-MAS\"es appears {} time(s).", cross_total);
        }
        Ok(())
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

use std::fs::File;
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
        if j+k >= x_len {
            break;
        }

        if wordsearch[i][j+k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if j+k >= x_len || i+k >= y_len {
            break;
        }

        if wordsearch[i+k][j+k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if i+k >= y_len {
            break;
        }

        if wordsearch[i+k][j] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    for k in 1..=3 {
        if j < k || i+k >= y_len {
            break;
        }

        if wordsearch[i+k][j-k] != word[k] {
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

        if wordsearch[i][j-k] != word[k] {
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

        if wordsearch[i-k][j-k] != word[k] {
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
        
        if wordsearch[i-k][j] != word[k] {
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

        if wordsearch[i-k][j+k] != word[k] {
            break;
        }

        if k == 3 {
            count += 1;
        }
    }

    count
}


fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(lines) = read_lines("./data/wordsearch.txt") {
        let wordsearch: Vec<Vec<u8>> = lines.flatten().map(|s| s.into_bytes()).collect();
        let mut total = 0;
        for (i, s) in wordsearch.iter().enumerate() {
            for (j, _c) in s.iter().enumerate() {
                total += count_xmas(i, j, &wordsearch)
            }
        }
        println!("The word \"XMAS\" appears {} time(s).", total);
    }
    Ok(())
}

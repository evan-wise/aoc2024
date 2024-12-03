use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader, Read};
use std::error::Error;

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

struct FileCharIterator {
    reader: BufReader<File>
}

impl FileCharIterator {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(FileCharIterator { reader })
    }
}

impl Iterator for FileCharIterator {
    type Item = io::Result<char>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => Some(Ok(buf[0] as char)),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

enum State {
    Seeking,
    MulFound,
    OpenParenFound,
    FirstNumFound,
    CommaFound,
    SecondNumFound,
    CloseParenFound,
}

fn main() -> Result<(), Box<dyn Error>> {
    let chars = FileCharIterator::new("./data/memory.txt")?;
    let mut state = State::Seeking;
    let mut temp = String::new();
    let mut first_num = 0;
    let mut second_num = 0;
    let mut total = 0;
    for char in chars.flatten() {
        match state {
            State::Seeking => {
                temp.push(char);
                if temp.len() > 3 {
                    temp = temp[1..].to_string();
                }
                if &temp == "mul" {
                    state = State::MulFound;
                    temp = String::new();
                }
            },
            State::MulFound => {
                if char == '(' {
                    state = State::OpenParenFound;
                } else {
                    state = State::Seeking;
                }
            },
            State::OpenParenFound => {
                if char.is_digit(10) {
                    state = State::FirstNumFound;
                    temp.push(char);
                } else {
                    state = State::Seeking;
                }
            },
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
                    temp = String::new();
                }
            },
            State::CommaFound => {
                if char.is_digit(10) {
                    state = State::SecondNumFound;
                    temp.push(char);
                } else {
                    state = State::Seeking;
                }
            },
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
                    temp = String::new();
                }
            },
            State::CloseParenFound => {
                temp.push(char);
                state = State::Seeking;
            },
        }
    }
    println!("The sum total of all the multiplications is: {}", total);
    Ok(())
}

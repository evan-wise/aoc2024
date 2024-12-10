use std::num::ParseIntError;

use crate::days::Solution;
use crate::aoc::{read_chars, FileCharIterator};

pub struct Day09;

impl Solution for Day09 {
    fn solve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let chars = read_chars("./data/day09.txt")?;
        let mut blocks = parse_input(chars)?;
        sort_blocks(&mut blocks);
        let checksum = compute_checksum(&blocks);
        println!("The checksum is: {}", checksum);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum Block {
    File(usize),
    Empty,
}

fn parse_input(chars: FileCharIterator) -> Result<Vec<Block>, ParseIntError> {
    let mut blocks = Vec::new();
    for (i, c) in chars.flatten().enumerate() {
        if c == '\n' {
            break;
        }
        let j = i / 2;
        let size = c.to_string().parse::<usize>()?;
        if i % 2 == 0 {
            blocks.append(&mut vec![Block::File(j); size]);
        } else {
            blocks.append(&mut vec![Block::Empty; size]);
        }
    }
    Ok(blocks)
}

fn sort_blocks(blocks: &mut Vec<Block>) {
    let mut i = 0;
    let mut j = blocks.len() - 1;
    while i < j {
        if let Block::File(_) = blocks[i] {
            i += 1;
        }
        if let Block::Empty = blocks[j] {
            j -= 1;
        }
        if let (Block::Empty, Block::File(_)) = (blocks[i], blocks[j]) {
            let swap = blocks[j];
            blocks[j] = blocks[i];
            blocks[i] = swap;
        }
    }
}

fn compute_checksum(blocks: &Vec<Block>) -> usize {
    let mut checksum = 0;
    for (i, block) in blocks.into_iter().enumerate() {
        if let Block::File(id) = block {
            checksum += i * id;
        }
    }
    checksum
}

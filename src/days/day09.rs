use crate::aoc::{read_chars, FileCharIterator, Solution, SolutionParts};
use std::error::Error;
use std::num::ParseIntError;

pub struct Day09;

impl Solution for Day09 {
    fn solve(&self) -> Result<SolutionParts, Box<dyn Error>> {
        let chars = read_chars("./data/day09.txt")?;
        let blocks = parse_input(chars)?;
        let mut blocks1 = blocks.clone();
        compact_by_block(&mut blocks1);
        let checksum1 = compute_checksum(&blocks1);
        let mut blocks2 = blocks.clone();
        compact_by_chunk(&mut blocks2);
        let checksum2 = compute_checksum(&blocks2);
        Ok((Some(checksum1.to_string()), Some(checksum2.to_string())))
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

fn compact_by_block(blocks: &mut Vec<Block>) {
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

fn compact_by_chunk(blocks: &mut Vec<Block>) {
    let mut end = blocks.len();
    while let Some(file_chunk) = find_file_chunk(&blocks, end) {
        let mut start = 0;
        while let Some(empty_chunk) = find_empty_chunk(&blocks, start) {
            if empty_chunk.0 > file_chunk.0 {
                break;
            }
            if empty_chunk.1 >= file_chunk.1 {
                for i in 0..file_chunk.1 {
                    let swap = blocks[file_chunk.0 + i];
                    blocks[file_chunk.0 + i] = blocks[empty_chunk.0 + i];
                    blocks[empty_chunk.0 + i] = swap;
                }
                break;
            } else {
                start = empty_chunk.0 + empty_chunk.1;
            }
        }
        end = file_chunk.0;
    }
}

fn find_empty_chunk(blocks: &Vec<Block>, start: usize) -> Option<(usize, usize)> {
    if start >= blocks.len() {
        return None;
    }
    let mut i = start;
    let mut j = start;
    while let Block::File(_) = blocks[i] {
        i += 1;
        j += 1;
        if i == blocks.len() {
            break;
        }
    }
    if i == blocks.len() {
        return None;
    }
    while let Block::Empty = blocks[j] {
        j += 1;
        if j == blocks.len() {
            break;
        }
    }
    Some((i, j - i))
}

fn find_file_chunk(blocks: &Vec<Block>, end: usize) -> Option<(usize, usize)> {
    if end > blocks.len() {
        return None;
    }
    if end == 0 {
        return None;
    }
    let mut i = end;
    let mut j = end;
    let mut maybe_file_id: Option<usize> = None;
    while let Block::Empty = blocks[i - 1] {
        i -= 1;
        j -= 1;
        if i == 0 {
            break;
        }
    }
    while let Block::File(id) = blocks[j - 1] {
        if let None = maybe_file_id {
            maybe_file_id = Some(id);
        }

        let file_id = maybe_file_id.unwrap();

        if file_id != id {
            break;
        }

        j -= 1;
        if j == 0 {
            break;
        }
    }
    if i == 0 {
        return None;
    }
    Some((j, i - j))
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

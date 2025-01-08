use crate::aoc::{read_chars, Answers, Solution};
use std::collections::VecDeque;
use std::error::Error;
use std::usize;

#[derive(Debug)]
pub struct Day09 {
    blocks: Vec<Block>,
    rle: Vec<(Block, usize, usize)>,
}

impl Day09 {
    pub fn new() -> Day09 {
        Day09 { blocks: Vec::new(), rle: Vec::new() }
    }
}

impl Solution for Day09 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let chars = read_chars("./data/day09.txt")?;
        let mut buff = VecDeque::new();
        for (i, c) in chars.flatten().enumerate() {
            if c == '\n' {
                if i % 2 != 0 {
                    let last = buff.len() - 1;
                    self.rle.push((Block::File((i - 1) / 2), buff[last], 0));
                }
                break;
            }
            let size = c.to_string().parse::<usize>()?;
            buff.push_back(size);
            if buff.len() > 2 {
                buff.pop_front();
            }
            if i % 2 == 0 {
                self.blocks.append(&mut vec![Block::File(i / 2); size]);
            } else {
                self.blocks.append(&mut vec![Block::Empty; buff[1]]);
                self.rle.push((Block::File(i / 2), buff[0], buff[1]));
            }
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        compact_by_block(&mut self.blocks);
        let checksum1 = compute_checksum(&self.blocks);
        compact_by_chunk2(&mut self.rle);
        let checksum2 = compute_checksum(&expand_rle(&self.rle));
        Ok(Answers::from(Some(checksum1), Some(checksum2)))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Block {
    File(usize),
    Empty,
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

fn expand_rle(rle: &Vec<(Block, usize, usize)>) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    for (block, file_size, empty_size) in rle {
        blocks.append(&mut vec![*block; *file_size]);
        blocks.append(&mut vec![Block::Empty; *empty_size]);
    }
    blocks
}

fn compact_by_chunk2(rle: &mut Vec<(Block, usize, usize)>) {
    let mut stack = (0..rle.len()).collect::<Vec<_>>();
    let n = rle.len();
    let mut i = n - 1;
    while let Some(id) = stack.pop() {
        while rle[i].0 != Block::File(id) {
            i -= 1;
        }
        let (_, file_size, empty_size) = rle[i];
        if let Some(j) = find_empty_space(rle, file_size, i) {
            rle.remove(i);
            let slot_size = rle[j].2;
            rle[j].2 = 0;
            rle.insert(j+1, (Block::File(id), file_size, slot_size - file_size));
            rle[i].2 += file_size + empty_size;
        } 
    }
}

fn find_empty_space(rle: &Vec<(Block, usize, usize)>, size: usize, max: usize) -> Option<usize> {
    let mut i = 0;
    for (_, _, empty_size) in rle {
        if i >= max {
            break;
        }
        if *empty_size >= size {
            return Some(i);
        }
        i += 1;
    }
    None
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

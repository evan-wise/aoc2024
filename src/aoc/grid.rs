use std::ops::{Index, IndexMut};

use crate::aoc::Position;

#[derive(Debug)]
pub struct Grid<T>
where T: Clone {
    items: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid { items: Vec::<T>::new(), width: 0, height: 0 }
    }

    pub fn fill(value:T, width: usize, height: usize) -> Grid<T> {
        Grid { items: vec![value; width * height], width, height }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        let (x, y) = position;
        if *x >= self.width || *y >= self.height {
            return None;
        }
        self.items.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, position: &Position) -> Option<&mut T> {
        let (x, y) = position;
        if *x >= self.width || *y >= self.height {
            return None;
        }
        self.items.get_mut(y * self.width + x)
    }
}

impl<T: Clone> Index<Position> for Grid<T> {
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        let (x, y) = index;
        if x >= self.width || y >= self.height {
            panic!();
        }
        self.items.index(y * self.width + x)
    }
}

impl<T: Clone> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        let (x, y) = index;
        if x >= self.width || y >= self.height {
            panic!();
        }
        self.items.index_mut(y * self.width + x)
    }
}

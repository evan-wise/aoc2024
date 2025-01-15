use std::ops::{Index, IndexMut};

use crate::aoc::Position;

#[derive(Debug)]
pub struct Grid<T>
where
    T: Clone,
{
    items: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl<T: Clone> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid {
            items: Vec::<T>::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn fill(value: T, width: usize, height: usize) -> Grid<T> {
        Grid {
            items: vec![value; width * height],
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, items: I) {
        self.items.extend(items);
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

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator::new(&self)
    }
}

impl<T: Clone> Extend<T> for Grid<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter);
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

pub struct GridIterator<'a, T>
where
    T: Clone,
{
    grid: &'a Grid<T>,
    i: usize,
}

impl<'a, T: Clone> GridIterator<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> GridIterator<'a, T> {
        GridIterator { grid, i: 0 }
    }
}

impl<'a, T: Clone> Iterator for GridIterator<'a, T> {
    type Item = (Position, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.grid.len() {
            return None;
        }
        self.i += 1;
        let x = self.i % self.grid.width;
        let y = self.i / self.grid.width;
        self.grid.items.get(self.i).cloned().map(|i| ((x, y), i))
    }
}

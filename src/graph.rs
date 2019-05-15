use crate::sudoku::{number::Number, Sudoku};
use std::ops::{Index, IndexMut};

pub struct Vertex {
    x: usize,
    y: usize,
    fixed: bool,
    value: Number,
}

impl Vertex {
    fn new(x: usize, y: usize, fixed: bool, value: Number) -> Self {
        Self { x, y, fixed, value }
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed
    }

    pub fn value(&self) -> Number {
        self.value
    }

    pub fn set_value(&mut self, value: Number) {
        self.value = value;
    }

    pub fn inc(&mut self) {
        self.value = self.value.inc();
    }

    pub fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub struct Graph {
    grid: [Vertex; 81],
}

impl Graph {
    pub fn new(puzzle: &Sudoku) -> Self {
        let mut grid: [Vertex; 81] = unsafe { std::mem::uninitialized() };

        for i in 0..9 {
            for j in 0..9 {
                let idx = 9 * i + j;
                let num = puzzle[(i, j)];
                let fixed = num == Number::Empty;

                let vertex = Vertex::new(i, j, fixed, num);
                grid[idx] = vertex;
            }
        }

        Self { grid }
    }
}

impl Index<(usize, usize)> for Graph {
    type Output = Vertex;

    fn index(&self, (x, y): (usize, usize)) -> &Vertex {
        let idx = x * 9 + y;
        &self.grid[idx]
    }
}

impl IndexMut<(usize, usize)> for Graph {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Vertex {
        let idx = x * 9 + y;
        &mut self.grid[idx]
    }
}

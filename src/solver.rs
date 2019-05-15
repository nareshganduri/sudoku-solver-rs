use crate::adj_mat::AdjacencyMatrix;
use crate::graph::Graph;
use crate::sudoku::{number::Number, Sudoku};

pub struct Solver {
    matrix: AdjacencyMatrix,
    graph: Graph,
}

impl Solver {
    pub fn new(puzzle: &Sudoku) -> Self {
        Self {
            matrix: AdjacencyMatrix::new(),
            graph: Graph::new(puzzle),
        }
    }

    fn num_works(&self, x: usize, y: usize) -> bool {
        let idx = 9 * x + y;
        for i in 0..81 {
            if !self.matrix[(idx, i)] {
                continue;
            }

            let (x2, y2) = (i / 9, i % 9);
            if self.graph[(x2, y2)].value() == self.graph[(x, y)].value() {
                return false;
            }
        }

        true
    }

    pub fn solve(mut self) -> Option<Sudoku> {
        let mut stack = vec![];
        let mut stack_idx = 0isize;

        for i in 0..9 {
            for j in 0..9 {
                if self.graph[(i, j)].is_fixed() {
                    stack.push((i, j));
                } else {
                    let valid = self.num_works(i, j);
                    if !valid {
                        return None;
                    }
                }
            }
        }

        while stack_idx >= 0 {
            let node = &mut self.graph[stack[stack_idx as usize]];
            if node.value() == Number::Nine {
                node.set_value(Number::Empty);
                stack_idx -= 1;
            } else {
                node.inc();

                let (x, y) = node.coords();
                let valid = self.num_works(x, y);
                if valid {
                    if stack_idx as usize == stack.len() - 1 {
                        break;
                    } else {
                        stack_idx += 1;
                        continue;
                    }
                } else {
                    continue;
                }
            }
        }

        Some(self.graph.into())
    }
}

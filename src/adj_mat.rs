use std::ops::{Index, IndexMut};

pub struct AdjacencyMatrix {
    matrix: [bool; 81 * 81],
}

impl AdjacencyMatrix {
    pub fn new() -> Self {
        let mut matrix = Self {
            matrix: [false; 81 * 81],
        };

        for i in 0..9 {
            for j in 0..9 {
                let curr_sq = i * 9 + j;

                for k in 0..9 {
                    let sq2 = i * 9 + k;

                    if curr_sq == sq2 {
                        continue;
                    }

                    matrix[(curr_sq, sq2)] = true;
                    matrix[(sq2, curr_sq)] = true;
                }

                for k in 0..9 {
                    let sq2 = k * 9 + j;

                    if curr_sq == sq2 {
                        continue;
                    }

                    matrix[(curr_sq, sq2)] = true;
                    matrix[(sq2, curr_sq)] = true;
                }
            }
        }

        for i in &[0, 3, 6] {
            for j in &[0, 3, 6] {
                for ii in *i..(*i + 3) {
                    for jj in *j..(*j + 3) {
                        let curr_sq = ii * 9 + jj;

                        for k in *i..(*i + 3) {
                            for l in *j..(*j + 3) {
                                let sq2 = k * 9 + l;

                                if sq2 == curr_sq {
                                    continue;
                                }

                                matrix[(curr_sq, sq2)] = true;
                                matrix[(sq2, curr_sq)] = true;
                            }
                        }
                    }
                }
            }
        }

        matrix
    }
}

impl Index<(usize, usize)> for AdjacencyMatrix {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &bool {
        &self.matrix[x * 81 + y]
    }
}

impl IndexMut<(usize, usize)> for AdjacencyMatrix {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut bool {
        &mut self.matrix[x * 81 + y]
    }
}
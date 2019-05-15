use self::number::Number;
use self::parser::{ParseError, SudokuParser};
use crate::graph::Graph;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Index;

pub mod number;
mod parser;

/// A struct representing a Sudoku puzzle in either solved or
/// unsolved form
pub struct Sudoku {
    solved: bool,
    grid: [Number; 81],
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..81 {
            if self.grid[i] != other.grid[i] {
                return false;
            }
        }

        true
    }
}

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.pretty_print();
        f.write_str(&s)?;

        Ok(())
    }
}

impl Sudoku {
    /// Attempts to construct a `Sudoku` puzzle using a string
    /// as input. Returns a `ParseError` if the input does not match
    /// the required format, which looks as follows:
    ///
    /// ```no_compile
    /// +---+---+---+
    /// | 4 |52 |7  |
    /// |1  | 3 | 8 |
    /// |  9|   |   |
    /// +---+---+---+
    /// | 5 | 7 |  9|
    /// |2 7|1  |   |
    /// |  6| 8 |4  |
    /// +---+---+---+
    /// |8  |  9| 5 |
    /// |  2|   | 1 |
    /// |7  |4  |9  |
    /// +---+---+---+
    /// ```
    ///
    /// Note that blank spaces can also be denoted by '0', 'x', 'X', or '_'.
    pub fn from_string(source: &str) -> Result<Sudoku, ParseError> {
        Self::try_from(source)
    }

    /// Attempts to construct a `Sudoku` puzzle from a list of numbers.
    /// An error is returned if the length of the list is not 81.
    /// Empty spaces are represented by 0's. Any number not in [0, 9]
    /// will return an error.
    ///
    /// The cells of the puzzle are filled in row-major order.
    pub fn from_numbers(nums: &[u8]) -> Result<Sudoku, NumParseError> {
        Self::try_from(nums)
    }

    /// Returns a `String` that could be used for pretty printing a `Sudoku`
    /// puzzle.
    ///
    /// As of the current implementation, the following holds:
    /// ```no_compile
    /// let s = sudoku.pretty_print();
    /// let puzzle = Sudoku::from_string(&s).unwrap();
    ///
    /// assert_eq!(sudoku, puzzle);
    /// ```
    pub fn pretty_print(&self) -> String {
        let mut s = String::new();
        let mut idx = 0;

        // write three squares
        for _ in 0..3 {
            s.push_str("+---+---+---+\n");

            // write three rows of numbers
            for _ in 0..3 {
                // write one row of numbers
                for _ in 0..3 {
                    s.push('|');

                    // write one number
                    for _ in 0..3 {
                        let c: u8 = self.grid[idx].into();
                        idx += 1;

                        if c == 0 {
                            s.push(' ');
                        } else {
                            let c = c + b'0';
                            s.push(c as char);
                        }
                    }
                }
                s.push_str("|\n");
            }
        }
        s.push_str("+---+---+---+\n");

        s
    }

    /// Returns whether the `Sudoku` is solved
    pub fn is_solved(&self) -> bool {
        self.solved
    }
}

impl Index<(usize, usize)> for Sudoku {
    type Output = Number;

    fn index(&self, (x, y): (usize, usize)) -> &Number {
        let idx = x * 9 + y;
        &self.grid[idx]
    }
}

impl From<Graph> for Sudoku {
    fn from(graph: Graph) -> Self {
        let mut grid = [Number::Empty; 81];
        let mut solved = true;

        for i in 0..9 {
            for j in 0..9 {
                let idx = i * 9 + j;
                let value = graph[(i, j)].value();
                if value == Number::Empty {
                    solved = false;
                }

                grid[idx] = value;
            }
        }

        Self { solved, grid }
    }
}

#[derive(Debug)]
pub enum NumParseError {
    NumberTooLarge,
    NotEnoughNumbers,
}

impl<'a> TryFrom<&'a [u8]> for Sudoku {
    type Error = NumParseError;

    fn try_from(nums: &'a [u8]) -> Result<Self, NumParseError> {
        if nums.len() != 81 {
            return Err(NumParseError::NotEnoughNumbers);
        }

        let mut grid = [Number::Empty; 81];
        let mut solved = true;

        for x in 0..9 {
            for y in 0..9 {
                let idx = x * 9 + y;
                if nums[idx] == 0 {
                    solved = false;
                } else {
                    let num = nums[idx]
                        .try_into()
                        .map_err(|_| NumParseError::NumberTooLarge)?;
                    grid[idx] = num;
                }
            }
        }

        Ok(Self { solved, grid })
    }
}

impl<'a> TryFrom<&'a str> for Sudoku {
    type Error = ParseError;

    fn try_from(source: &'a str) -> Result<Self, ParseError> {
        let parser = SudokuParser::new(source);

        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let my_string = "\
+---+---+---+
| 4 |52 |7  |
|1  | 3 | 8 |
|  9|   |   |
+---+---+---+
| 5 | 7 |  9|
|2 7|1  |   |
|  6| 8 |4  |
+---+---+---+
|8  |  9| 5 |
|  2|   | 1 |
|7  |4  |9  |
+---+---+---+";
        let my_nums = [
            0, 4, 0, 5, 2, 0, 7, 0, 0, 1, 0, 0, 0, 3, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 5,
            0, 0, 7, 0, 0, 0, 9, 2, 0, 7, 1, 0, 0, 0, 0, 0, 0, 0, 6, 0, 8, 0, 4, 0, 0, 8, 0, 0, 0,
            0, 9, 0, 5, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 7, 0, 0, 4, 0, 0, 9, 0, 0,
        ];

        let puzzle1 = Sudoku::from_string(my_string).expect("Parse failure!");
        let puzzle2 = Sudoku::from_numbers(&my_nums).expect("Parse failure!");

        assert_eq!(puzzle1.is_solved(), false);
        assert_eq!(puzzle2.is_solved(), false);
        assert_eq!(puzzle1, puzzle2);
    }
}

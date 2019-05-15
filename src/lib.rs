//! Sudoku solving library using backtracking

#![warn(clippy::all)]
#![warn(missing_docs)]

pub use self::sudoku::Sudoku;
use self::solver::Solver;

mod adj_mat;
mod graph;
mod solver;
mod sudoku;

/// Solves a Sudoku puzzle. Returns `None` if no solution is possible.
/// # Example
/// ```
/// # use sudoku_solver::solve;
/// # use sudoku_solver::Sudoku;
/// let unsolved = "\
/// +---+---+---+
/// |53 | 7 |   |
/// |6  |195|   |
/// | 98|   | 6 |
/// +---+---+---+
/// |8  | 6 |  3|
/// |4  |8 3|  1|
/// |7  | 2 |  6|
/// +---+---+---+
/// | 6 |   |28 |
/// |   |419|  5|
/// |   | 8 | 79|
/// +---+---+---+";
/// let solved = "\
/// +---+---+---+
/// |534|678|912|
/// |672|195|348|
/// |198|342|567|
/// +---+---+---+
/// |859|761|423|
/// |426|853|791|
/// |713|924|856|
/// +---+---+---+
/// |961|537|284|
/// |287|419|635|
/// |345|286|179|
/// +---+---+---+";
///
/// let puzzle = Sudoku::from_string(unsolved).expect("Parse failure!");
/// let expected = Sudoku::from_string(solved).expect("Parse failure!");
///
/// let actual = solve(&puzzle).expect("Could not solve puzzle!");
/// assert_eq!(actual, expected);
/// ```
pub fn solve(puzzle: &Sudoku) -> Option<Sudoku> {
    let solver = Solver::new(puzzle);

    solver.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let unsolved = "\
+---+---+---+
|53 | 7 |   |
|6  |195|   |
| 98|   | 6 |
+---+---+---+
|8  | 6 |  3|
|4  |8 3|  1|
|7  | 2 |  6|
+---+---+---+
| 6 |   |28 |
|   |419|  5|
|   | 8 | 79|
+---+---+---+";
        let solved = "\
+---+---+---+
|534|678|912|
|672|195|348|
|198|342|567|
+---+---+---+
|859|761|423|
|426|853|791|
|713|924|856|
+---+---+---+
|961|537|284|
|287|419|635|
|345|286|179|
+---+---+---+";

        let puzzle = Sudoku::from_string(unsolved).expect("Parse failure!");
        let expected = Sudoku::from_string(solved).expect("Parse failure!");

        let actual = solve(&puzzle).expect("Could not solve puzzle!");
        assert_eq!(actual, expected);
        assert_eq!(puzzle.is_solved(), false);
        assert_eq!(actual.is_solved(), true);
        assert_eq!(expected.is_solved(), true);
    }
}
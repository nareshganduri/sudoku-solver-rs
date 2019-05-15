use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read};
use sudoku_solver::{solve, Sudoku};

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    ParseError,
    Unsolvable,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

fn get_source(filename: &str) -> Result<String, io::Error> {
    let mut file = OpenOptions::new().read(true).open(filename)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    Ok(source)
}

fn get_solution(source: &str) -> Result<Sudoku, Error> {
    let puzzle = Sudoku::from_string(&source).map_err(|_| Error::ParseError)?;
    let solution = solve(&puzzle).ok_or(Error::Unsolvable)?;

    Ok(solution)
}

fn main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: solver.exe [file]");
        return Ok(());
    }

    let filename = &args[1];
    let source = get_source(filename)?;
    let solution = get_solution(&source)?;

    let pretty_print = solution.pretty_print();
    println!("{}", pretty_print);

    Ok(())
}

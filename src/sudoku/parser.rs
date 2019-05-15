use super::{Number, Sudoku};
use std::convert::TryInto;

#[derive(Debug)]
enum ErrorKind {
    Expected { found: u8, needed: u8 },
    MissingNewline,
    NotADigit,
}

#[derive(Debug)]
pub struct ParseError {
    line_no: usize,
    pos: usize,
    kind: ErrorKind,
}

pub struct SudokuParser<'a> {
    source: &'a [u8],
    line_no: usize,
    pos: usize,
    idx: usize,
    grid: [Number; 81],
    solved: bool,
}

impl<'a> SudokuParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            line_no: 1,
            pos: 0,
            idx: 0,
            grid: [Number::Empty; 81],
            solved: true,
        }
    }

    fn error(&self, kind: ErrorKind) -> ParseError {
        ParseError {
            line_no: self.line_no,
            pos: self.pos,
            kind,
        }
    }

    fn peek(&self) -> u8 {
        self.source[self.pos]
    }

    fn matches(&mut self, c: u8) -> bool {
        if self.peek() == c {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, c: u8) -> Result<(), ParseError> {
        if !self.matches(c) {
            Err(self.error(ErrorKind::Expected {
                found: self.peek(),
                needed: c,
            }))
        } else {
            Ok(())
        }
    }

    fn expect_newline(&mut self) -> Result<(), ParseError> {
        if self.matches(b'\r') {
            self.matches(b'\n');
            self.line_no += 1;
            Ok(())
        } else if self.matches(b'\n') {
            self.line_no += 1;
            Ok(())
        } else {
            Err(self.error(ErrorKind::MissingNewline))
        }
    }

    fn expect_line(&mut self, force_newline: bool) -> Result<(), ParseError> {
        self.expect(b'+')?;
        for _ in 0..3 {
            for _ in 0..3 {
                self.expect(b'-')?;
            }
            self.expect(b'+')?;
        }

        if force_newline {
            self.expect_newline()?;
        }

        Ok(())
    }

    fn expect_digit(&mut self) -> Result<(), ParseError> {
        match self.peek() {
            b' ' | b'x' | b'X' | b'_' => {
                self.solved = false;
            }
            x => {
                let digit = x - b'0';
                if digit == 0 {
                    self.solved = false;
                }
                
                self.grid[self.idx] = digit
                    .try_into()
                    .map_err(|_| self.error(ErrorKind::NotADigit))?;
            }
        }
        self.pos += 1;
        self.idx += 1;

        Ok(())
    }

    fn expect_row(&mut self) -> Result<(), ParseError> {
        self.expect(b'|')?;

        for _ in 0..3 {
            for _ in 0..3 {
                self.expect_digit()?;
            }
            self.expect(b'|')?;
        }
        self.expect_newline()?;

        Ok(())
    }

    pub fn parse(mut self) -> Result<Sudoku, ParseError> {
        for _ in 0..3 {
            self.expect_line(true)?;

            for _ in 0..3 {
                self.expect_row()?;
            }
        }
        self.expect_line(false)?;

        Ok(Sudoku {
            solved: self.solved,
            grid: self.grid,
        })
    }
}

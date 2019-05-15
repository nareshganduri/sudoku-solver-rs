# sudoku-solver-rs
A Rust port of my old [JS backtracking sudoku solver][1].
Presumably, the performance should be a bit better as native code
over plain JS, but I haven't run any benchmarks.

## Usage
The following should work:
```
cargo run --release -- [filename]
```
There are two examples in the [examples][2] folder. One should be 
solved fairly quickly and the other is designed to work against the 
backtracking solver, so it should take a while.

[1]: https://github.com/nareshganduri/sudokusolver
[2]: ./examples
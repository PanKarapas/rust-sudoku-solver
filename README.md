# Rust Sudoku Solver
This is a sudoku solver implemented in Rust. It currently supports a fairly simple back tracking solver. I plan to add smarter solvers in the future.

This is a learning project for me, as I have never used Rust before, so the code quality might not be perfect.

## Usage

Clone the repo and run:

```bash
cargo run --release
```
##  Benchmarks
These are measured in my personal machine, so YMMV. That said, they are all run on the same hardware, with the same version of the code, so should at least reflect the efficiency of each algorithm against the others.

A full historical list of bench mark results can be found at [bench_results.md](/benches/bench_results.md).

### Latest

Name | [fastest, median, slowest]
--- | --- 
Backtracking_simple | [2.0409 ms 2.0721 ms 2.1133 ms] 
Backtracking_hard | [6339.5 s 6419.1 s 6535.0 s]

### Usage
```bash
cargo bench
```

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
Backtracking_simple | [0.62312 ms 0.62718 ms 0.63191 ms]
Backtracking_hard | [1948.5 ms 1974.3 ms 2000.0 ms]
ConstrainedBackTrackingSolver_simple | [0.19662 ms 0.20070 ms 0.20518 ms]
ConstrainedBackTrackingSolver_hard | [1105.7 ms 1114.6 ms 1123.9 ms]

### Usage
```bash
cargo bench
```

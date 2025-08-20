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
Backtracking_simple | [0.61103 ms 0.61214 ms 0.61343 ms]
Backtracking_hard | [1844.7 ms 1854.6 ms 1864.9 ms]
ConstrainedBackTrackingSolver_simple | [0.20164 ms 0.20315 ms 0.20513]
ConstrainedBackTrackingSolver_hard | [1041.0 ms 1048.3 ms 1057.2 ms]


### Usage
```bash
cargo bench
```

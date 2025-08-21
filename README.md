# Rust Sudoku Solver
This is a sudoku solver implemented in Rust. It currently supports a fairly simple back tracking solver. I plan to add smarter solvers in the future.

This is a learning project for me, as I have never used Rust before, so the code quality might not be perfect.

## Usage

Clone the repo and run:

```bash
cargo run --release
```
## Solvers
There are currently 2 solvers:
### Backtracking

A simple, straight forward backtracking algorithm. It does not apply any rules to the sudoku, just fills in the board from the top left to the bottom right one number at a time. If at any point the board is invalid, it goes back and increments the last number it set. If the last cell set is a 9, it will go to the cell before that.

### Cell Eliminated Backtracking
A slightly more sophisticated version of the backtracking algorithm. Before it starts backtracking, it does a pre-process pass where it finds any cells that can only take one value (based on the puzzle's fixed numbers) and sets them. It also keeps track of which values each cell can take based on the fixed values. The pre-processor can run multiple times, until the board is in a stable position where no more values can be set.

Once the pre-process step is done, a very similar algorithm runs as in the simple backtracking solver. The only difference is that we skip over testing any values we know are not allowed for the cell.

##  Benchmarks
These are measured in my personal machine, so YMMV. That said, they are all run on the same hardware, with the same version of the code, so should at least reflect the efficiency of each algorithm against the others.

A full historical list of bench mark results can be found at [bench_results.md](/benches/bench_results.md).

The name comes from the fact that for each cell we eliminate all the values it can't be, and that allows us to infer which values it can take.

### Latest
Name | [fastest, median, slowest]
--- | --- 
Backtracking_simple | [0.62312 ms 0.62718 ms 0.63191 ms]
Backtracking_hard | [1948.5 ms 1974.3 ms 2000.0 ms]
CellEliminatedBacktrackingSolver_simple | [0.19662 ms 0.20070 ms 0.20518 ms]
CellEliminatedBacktrackingSolver_hard | [1105.7 ms 1114.6 ms 1123.9 ms]

### Usage
```bash
cargo bench
```

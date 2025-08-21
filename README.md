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

The name of the solver comes from the fact that we check which values are eliminated for that cell, and if there is only 1, we choose it.

### Group Eliminated Backtracking
Similar to the Cell Eliminated version, this solver tries to find some easy cell solutions before going back to back tracking. For this solver the pre-processor looks at each non-fixed cell and every other cell in its groups (column/row/square). If this cell is the only one that can take a specific value, and the value is allowed in the cell, then we know the cell must have that value.

As an example, if cell (0, 0) of a row is the only cell in that row that can be a 2, then we know it must be 2 for the puzzle to be solvable.

As with the Cell Eliminated Solver, this runs until we come to a stable board, which is then passed to the normal back tracking implementation (still skipping any values that the cell is forbidden from using).

The name of the solver comes from the fact that we check which value in its groups the cell has to take, and we choose it. So we check which values are eliminated for all cells in its groups, and we select the one that is eliminated for all but the current cell.

Note: This is more processing intensive than the Cell Eliminated pre-processor. Depending on the puzzle this approach can be faster or slower (as you can see in the benchmarks). 

##  Benchmarks
These are measured in my personal machine, so YMMV. That said, they are all run on the same hardware, with the same version of the code, so should at least reflect the efficiency of each algorithm against the others.

A full historical list of bench mark results can be found at [bench_results.md](/benches/bench_results.md).

The name comes from the fact that for each cell we eliminate all the values it can't be, and that allows us to infer which values it can take.

### Latest
Name | [fastest, median, slowest]
--- | --- 
Backtracking_simple | [0.66815 ms 0.68755 ms 0.70721 ms]
Backtracking_hard | [1833.0 ms 1867.5 ms 1902.8 ms]
CellEliminatedBacktrackingSolver_simple | [0.20179 ms 0.20415 ms 0.20687 ms]
CellEliminatedBacktrackingSolver_hard | [1116.2 ms 1120.3 ms 1125.0 ms]
GroupEliminatedBacktrackingSolver_simple | [0.43074 ms 0.43724 ms 0.44520 ms]
GroupEliminatedBacktrackingSolver_hard | [659.86 ms 672.97 ms 688.00 ms]

### Usage
```bash
cargo bench
```

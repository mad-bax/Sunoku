# Sunoku

It solves Sudoku puzzles!

## Compilation

```bash
cargo build --release
```

## The Solver

The solver can solve puzzles whose sides are some square number. I.e., 9, 16, 25, 36, ... etc. It may be able to solve 'non square' board sizes but I have not tested thoroughly.

There are two algorithms given, a `Naive` method and my personal strategy made algorithmic, nicknamed `BaxStrat`. The Naive method is a simple brute force which takes and $O(n^3)$ time to complete where $n$ is the size of the board. The second method `BaxStrat` iterates over the board and looks for various patterns with the board values to eliminate possibilities before handing off the board to the Naive solver. At worst it has the same performance as the Naive approach though it generally out performs Naive for most boards.

## Input File Formats

The board file inputs are relatively simple. The file inputs are decimal values separated by whitespace, where the very first value is the $n$ length of the board. The solver then expects $n^2$ values after and will exit early if it is not *exactly* so.

Zeros represent 'unassigned' positions.

### Whitespace

Whitespace is used as the delimiter of values and it does not matter what amount of whitespaces there are or what type.

Therefore any of the following formats are acceptable for the same 16x16 board:

#### Minimal

```text
16
9 13 12 0 0 0 14 11 0 0 6 4 16 10 0 3
10 0 0 1 6 0 0 0 13 0 2 0 0 8 0 14
0 0 0 14 8 0 0 0 0 12 9 0 6 15 1 0
0 0 6 0 13 15 1 0 10 14 0 0 12 4 0 0
7 0 0 0 2 0 4 13 0 15 0 0 0 6 0 1
13 16 11 0 0 1 0 0 0 0 4 14 0 0 12 15
6 1 0 0 0 0 11 15 8 13 7 0 0 0 4 16
0 0 8 5 0 0 6 7 0 16 0 1 0 14 0 0
0 0 10 6 14 13 0 5 0 3 0 0 0 0 16 2
5 0 0 7 0 12 0 0 14 0 0 0 4 3 0 9
15 0 13 0 0 0 3 0 0 0 5 2 0 0 0 0
16 3 2 0 0 8 15 0 0 0 1 6 0 0 14 10
3 0 0 11 0 0 0 0 0 0 14 0 10 0 9 0
0 0 0 0 0 0 0 0 9 2 0 0 0 5 3 6
0 10 9 8 1 0 0 2 0 0 0 0 0 16 13 4
14 0 15 0 0 0 0 0 5 1 13 10 0 2 0 7
```

#### Formated

```text
16
9  13 12 0   0  0  14 11  0  0  6  4   16 10 0 3
10 0  0  1   6  0  0  0   13 0  2  0   0  8  0 14
0  0  0  14  8  0  0  0   0  12 9  0   6  15 1 0
0  0  6  0   13 15 1  0   10 14 0  0   12 4  0 0

7  0  0  0   2  0  4  13  0  15 0  0   0  6  0  1
13 16 11 0   0  1  0  0   0  0  4  14  0  0  12 15
6  1  0  0   0  0  11 15  8  13 7  0   0  0  4  16
0  0  8  5   0  0  6  7   0  16 0  1   0  14 0  0

0  0  10 6   14 13 0  5   0  3  0  0   0  0  16 2
5  0  0  7   0  12 0  0   14 0  0  0   4  3  0  9
15 0  13 0   0  0  3  0   0  0  5  2   0  0  0  0
16 3  2  0   0  8  15 0   0  0  1  6   0  0  14 10

3  0  0  11  0  0  0  0   0  0  14 0   10 0  9  0
0  0  0  0   0  0  0  0   9  2  0  0   0  5  3  6
0  10 9  8   1  0  0  2   0  0  0  0   0  16 13 4
14 0  15 0   0  0  0  0   5  1  13 10  0  2  0  7
```

#### Sequential

```text
16 9 13 12 0 0 0 14 11 0 0 6 4 16 10 0 3 10 0 0 1 6 0 0 0 13 0 2 0 0 8 0 14 0 0 0 14 8 0 0 0 0 12 9 0 6 15 1 0 0 0 6 0 13 15 1 0 10 14 0 0 12 4 0 0 7 0 0 0 2 0 4 13 0 15 0 0 0 6 0 1 13 16 11 0 0 1 0 0 0 0 4 14 0 0 12 15 6 1 0 0 0 0 11 15 8 13 7 0 0 0 4 16 0 0 8 5 0 0 6 7 0 16 0 1 0 14 0 0 0 0 10 6 14 13 0 5 0 3 0 0 0 0 16 2 5 0 0 7 0 12 0 0 14 0 0 0 4 3 0 9 15 0 13 0 0 0 3 0 0 0 5 2 0 0 0 0 16 3 2 0 0 8 15 0 0 0 1 6 0 0 14 10 3 0 0 11 0 0 0 0 0 0 14 0 10 0 9 0 0 0 0 0 0 0 0 0 9 2 0 0 0 5 3 6 0 10 9 8 1 0 0 2 0 0 0 0 0 16 13 4 14 0 15 0 0 0 0 0 5 1 13 10 0 2 0 7
```


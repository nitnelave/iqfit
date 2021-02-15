# iqfit
Rust solver for the IQ Fit puzzle game

## Benchmarks

| Change                   | Commit  | Puzzle 49 | Puzzle 117 |
| ------------------------ | ------  | --------- | ---------- |
| Initial                  | 06c9d24 | 31 us     | 111 ms     |
| Lower bound hint         | 488a43f | 30 us     | 109 ms     |
| Face distribution cutoff | dba15d3 | 22 us     | 99 ms      |
| Binary board             | 3cfb02a | 19 us     | 84 ms      |
| Empty cell lookup table  | c2106d2 | 19 us     | 84 ms      |

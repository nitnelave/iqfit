# iqfit

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Rust solver for the IQ Fit puzzle game.

## Benchmarks

| Change                     | Commit  | Puzzle 49 | Puzzle 117 |
| -------------------------- | ------  | --------- | ---------- |
| Initial                    | 06c9d24 | 31 us     | 111 ms     |
| Lower bound hint           | 488a43f | 30 us     | 109 ms     |
| Face distribution cutoff   | dba15d3 | 22 us     | 99 ms      |
| Binary board               | 3cfb02a | 19 us     | 84 ms      |
| Empty cell lookup table    | c2106d2 | 19 us     | 84 ms      |
| Bitfield color set         | 1676d54 | 5.10 us   | 29.0 ms    |
| Misc micro-opti            | f48a455 | 4.86 us   | 28.3 ms    |
| Detect easy failure case   | 2649173 | 4.18 us   | 22.7 ms    |
| More failure cases         | 6d6afde | 4.09 us   | 21.9 ms    |
| Make board immutable       | 501c372 | 7.61 us   | 6.75 ms    |
| Search patterns everywhere | 84584a7 | 3.52 us   | 2.05 ms    |
| Add missing pattern        | b38e44d | 3.16 us   | 1.72 ms    |
| Re-order board checks      | 12c5345 | 3.12 us   | 1.53 ms    |
| Fuse board cond & update   | 0377fde | 3.10 us   | 1.50 ms    |

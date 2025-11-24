# Advent of Code - Rust Solutions

This repository contains my solutions to [Advent of Code](https://adventofcode.com/) challenges implemented in Rust.

## Project Structure

```bash
advent-of-code-rust/
├── src/
│   ├── main.rs           # Entry point - runs all solutions
│   ├── lib.rs            # Library root - exports modules
│   ├── helpers/          # Common helper functions
│   │   └── mod.rs        # Utility functions (file reading, parsing, etc.)
│   ├── year2015/         # Solutions for 2015
│   │   ├── mod.rs        # Year module
│   │   ├── day1.rs       # Day 1 solution
│   │   └── day2.rs       # Day 2 solution
│   └── year2024/         # Solutions for 2024
│       └── ...
├── inputs/               # Input files
│   ├── 2015/
│   │   ├── 1.in         # Day 1 input
│   │   └── 2.in         # Day 2 input
│   └── 2024/
│       └── ...
└── Cargo.toml           # Project dependencies
```

## License

See LICENSE file.

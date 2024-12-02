<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2024

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `100.3µs` | `90.0µs` |
| [Day 2](./src/bin/02.rs) | `168.0µs` | `732.8µs` |

**Total: 1.09ms**
<!--- benchmarking table --->

---

## Usage

### ➡️ Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

### ➡️ Download input for a day

> [!IMPORTANT] 
> This requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

You can automatically download puzzle input and description by either appending the `--download` flag to `scaffold` (e.g. `cargo scaffold 4 --download`) or with the separate `download` command:

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### ➡️ Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

### ➡️ Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

### ➡️ Benchmark your solutions

```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

The `cargo time` command allows you to benchmark your code and store timings in the readme. When benching, the runner will run your code between `10` and `10.000` times, depending on execution time of first execution, and print the average execution time.

`cargo time` has three modes of execution:

 1. `cargo time` without arguments incrementally benches solutions that do not have been stored in the readme yet and skips the rest.
 2. `cargo time <day>` benches a single solution.
 3. `cargo time --all` benches all solutions.

By default, `cargo time` does not write to the readme. In order to do so, append the `--store` flag: `cargo time --store`.

> Please note that these are not _scientific_ benchmarks, understand them as a fun approximation. 😉 Timings, especially in the microseconds range, might change a bit between invocations.

### ➡️ Run all tests

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### ➡️ Read puzzle description

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### ➡️ Scaffold, download & read the current aoc day

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

During december, the `today` shorthand command can be used to:

 - scaffold a solution for the current day
 - download its input
 - and read the puzzle

in one go.

```sh
# example: `cargo today` on December 1st
cargo today

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
#
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

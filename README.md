# AoC 2024 Solutions

These are my problem solutions for the challenge, presented for general
interest. Some are quite messy as I am still learning Rust. There are also some
attempts at building out a library to reuse for future years. These are probably
crap but will help me build the real version better.

## Build

This is a fairly standard project. To build it, run `cargo build -r`.

## Running the Solutions

Run all the solutions with `cargo run -r` or run a specific solution with `cargo
run -r <num>`. The program also prints out some very rough performance
measurements.

## Todo

- Review solutions for common patterns
- Clean up any messy solutions
- Continue optimizing slowest solutions
- Command-line option to specify example mode (`-e`?)
- Command-line option to pass a custom input file (`-f`)
- Add parallelism


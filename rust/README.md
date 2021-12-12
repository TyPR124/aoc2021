# Advent of Code 2021 - Rust

Rust solutions to Advent of Code 2021 challenge.

See: https://adventofcode.com/2021

Solutions will generally prefer performance over style.

## Running it

To run and/or test the solutions, you will need the Rust toolchain for your system.

See https://rustup.rs/ for information on installing Rust.

The binary for each day will automatically download the day's input using your adventofcode.com session cookie. This cookie is read from the run-time environment variable `AOC_SESSION_COOKIE`. You can find your session cookie within your browser after having logged into adventofcode.com. You may set the environment variable in any way you see fit. Setting the variable via a [.env file](https://docs.rs/dotenv/latest/dotenv/) is supported: `echo AOC_SESSION_COOKIE=$my_session_cookie_value >> .env`.

To run the solution for day 1, issue the command `cargo run --bin day1`.

To test the solution for day 1, issue the command `cargo test --bin day1`.

To test all solutions, issue the command `cargo test`.

Note that while the code usually tries to be performant, by default cargo does not optimize the compiled binary. To enable optimization, include the `--release` flag, for example `cargo run --bin day1 --release`. Building a relase mode binary can take significantly more time than an unoptimized binary.

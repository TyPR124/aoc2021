use std::{
    fmt::{Debug, Display},
    time::Instant,
};

use anyhow::{Context, Result};

/// Common functionality for a day's solution.
pub trait Solution {
    /// The day number
    const DAY: u8;
    /// The concrete type returned for part 1
    type Out1: Display + Debug;
    /// The concrete type returned for part 2
    type Out2: Display + Debug;

    /// A function which should solve both parts and return the correct values
    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)>;
}

/// Retrieve the real input for the given day.
///
/// Requires a run-time environment variable named `AOC_SESSION_COOKIE`.
/// If this variable is not set, the function will fail.
pub fn get_input(day: u8) -> Result<String> {
    if dotenv::dotenv().is_err() {
        eprintln!("WARN: failed to load .env file")
    }

    let session_cookie_value = std::env::var("AOC_SESSION_COOKIE")
        .context("failed to read AOC_SESSION_COOKIE from environment")?;

    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let data = ureq::get(&url)
        .set("Cookie", &format!("session={}", session_cookie_value))
        .call()
        .context("http request error")?
        .into_string()
        .context("http response error")?;
    Ok(data)
}

/// Run the solution and print the results.
pub fn run_solution<S: Solution>() -> Result<()> {
    let input = get_input(S::DAY)
        .with_context(|| format!("failed to retrieve input for day {:02}", S::DAY))?;

    let time = Instant::now();

    let (part1, part2) = S::solve(input)?;

    let time = time.elapsed();

    println!("Day {:02} - Part 1: {}", S::DAY, part1);
    println!("Day {:02} - Part 2: {}", S::DAY, part2);
    println!(
        "Run took {:.5}s | {}ms | {}μs ({})",
        time.as_secs_f32(),
        time.as_millis(),
        time.as_micros(),
        if cfg!(debug_assertions) {
            "DEBUG"
        } else {
            "RELEASE"
        }
    );

    Ok(())
}

/// Test the solution, given specific input and the expected answers.
///
/// Will panic on error or incorrect output.
pub fn test_solution<S, I, A1, A2>(input: I, (answer1, answer2): (A1, A2))
where
    S: Solution,
    I: Into<String>,
    A1: Debug,
    A2: Debug,
    S::Out1: PartialEq<A1> + Debug,
    S::Out2: PartialEq<A2> + Debug,
{
    println!("Testing day {:02}", S::DAY);
    let input = input.into();
    println!(
        "Input (starts on next line):\n{}\n(input ends on line above)",
        input
    );

    let result = S::solve(input);
    println!("Got Result: {:?}", result);
    let (part1, part2) = result.unwrap();
    assert_eq!(part1, answer1, "Part 1 failure");
    assert_eq!(part2, answer2, "Part 2 failure");
}

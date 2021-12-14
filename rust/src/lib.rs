use std::{
    cmp,
    fmt::{Debug, Display},
    iter::{self, FusedIterator},
    ops::{Index, IndexMut},
    slice,
    time::Instant,
};

use anyhow::{ensure, Context, Result};

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

    println!("Day {:02} - Part 1:\n{}", S::DAY, part1);
    println!("Day {:02} - Part 2:\n{}", S::DAY, part2);
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

#[derive(Clone, Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    size: GridSize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GridSize {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GridIndex {
    pub index: usize,
    pub size: GridSize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, size: GridSize) -> Result<Self> {
        ensure!(
            data.len() == size.to_len(),
            "data length does not match dimensions in grid construction"
        );
        Ok(Self { data, size })
    }
    pub fn size(&self) -> GridSize {
        self.size
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn iter(&self) -> slice::Iter<T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.data.iter_mut()
    }
    pub fn iter_copied(&self) -> iter::Copied<slice::Iter<T>>
    where
        T: Copy,
    {
        self.data.iter().copied()
    }
}

impl<T, I: Into<usize>> Index<I> for Grid<T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        self.data.index(index.into())
    }
}

impl<T, I: Into<usize>> IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.data.index_mut(index.into())
    }
}

impl GridSize {
    pub fn to_len(self) -> usize {
        self.width * self.height
    }
}

impl GridIndex {
    pub fn with_index(self, index: usize) -> Self {
        Self { index, ..self }
    }
    pub fn to_usize(self) -> usize {
        self.index
    }
    pub fn x(self) -> usize {
        self.index % self.size.width
    }
    pub fn y(self) -> usize {
        self.index / self.size.width
    }
    pub fn row(self) -> usize {
        self.y()
    }
    pub fn col(self) -> usize {
        self.x()
    }
    pub fn neighbors(self, kind: Neighbors) -> [Option<Self>; 8] {
        use Neighbors::*;
        let Self { index, size } = self;
        let GridSize { width, height } = size;
        let (x, y) = (self.x(), self.y());
        let left = x > 0;
        let right = x + 1 < width;
        let top = y > 0;
        let bottom = y + 1 < height;

        [
            // Top Left
            (matches!(kind, All | Ordinal | Backward) && left && top)
                .then(|| self.with_index(index - width - 1)),
            // Top Center
            (matches!(kind, All | Cardinal | Backward) && top)
                .then(|| self.with_index(index - width)),
            // Top Right
            (matches!(kind, All | Ordinal | Backward) && top && right)
                .then(|| self.with_index(index - width + 1)),
            // Left Center
            (matches!(kind, All | Cardinal | Backward) && left).then(|| self.with_index(index - 1)),
            // Right Center
            (matches!(kind, All | Cardinal | Forward) && right).then(|| self.with_index(index + 1)),
            // Bottom Left
            (matches!(kind, All | Ordinal | Forward) && left && bottom)
                .then(|| self.with_index(index + width - 1)),
            // Bottom Center
            (matches!(kind, All | Cardinal | Forward) && bottom)
                .then(|| self.with_index(index + width)),
            // Bottom Right
            (matches!(kind, All | Ordinal | Forward) && right && bottom)
                .then(|| self.with_index(index + width + 1)),
        ]
    }

    pub fn neighbors_iter(
        self,
        kind: Neighbors,
    ) -> impl Iterator<Item = Self> + DoubleEndedIterator + FusedIterator + Clone + Debug {
        self.neighbors(kind).into_iter().flatten()
    }
}

pub enum Neighbors {
    Cardinal,
    Ordinal,
    Forward,
    Backward,
    All,
}

impl From<GridIndex> for usize {
    fn from(i: GridIndex) -> Self {
        i.index
    }
}

pub trait Ascii {
    fn to_digit(self) -> Option<u8>;
    fn to_lowercase_index(self) -> Option<u8>;
    fn to_uppercase_index(self) -> Option<u8>;
}

impl Ascii for u8 {
    fn to_digit(self) -> Option<u8> {
        (b'0'..=b'9').contains(&self).then(|| self - b'0')
    }
    fn to_lowercase_index(self) -> Option<u8> {
        (b'a'..=b'z').contains(&self).then(|| self - b'a')
    }
    fn to_uppercase_index(self) -> Option<u8> {
        (b'A'..=b'Z').contains(&self).then(|| self - b'A')
    }
}

pub trait IterTools
where
    Self: Iterator + Sized,
{
    fn min_max(self) -> Option<(Self::Item, Self::Item)>
    where
        Self::Item: Ord,
    {
        self.min_max_by(Ord::cmp)
    }
    fn min_max_by_key<K, F>(self, mut f: F) -> Option<(Self::Item, Self::Item)>
    where
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        let ((_kmin, min), (_kmax, max)) = self
            .map(|x| (f(&x), x))
            .min_max_by(|(ka, _), (kb, _)| ka.cmp(kb))?;
        Some((min, max))
    }
    fn min_max_by<F>(mut self, mut compare: F) -> Option<(Self::Item, Self::Item)>
    where
        F: FnMut(&Self::Item, &Self::Item) -> cmp::Ordering,
    {
        use cmp::Ordering::*;
        let first = self.next()?;
        let second = self.next()?;
        let (min, max) = match compare(&first, &second) {
            Less | Equal => (first, second),
            Greater => (second, first),
        };
        let (min, max) = self.fold((min, max), |(min, max), new| match compare(&new, &min) {
            Less => (new, max),
            Equal => (min, max),
            Greater => match compare(&new, &max) {
                Greater => (min, new),
                _ => (min, max),
            },
        });
        Some((min, max))
    }
}

impl<I> IterTools for I where I: Iterator {}

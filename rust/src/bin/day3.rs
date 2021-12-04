#![feature(drain_filter)]

use std::iter;

use anyhow::{Context, Result};

#[test]
fn test_day3() {
    const INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
    aoc2021::test_solution::<Day3, _, _, _>(INPUT, (198, 230));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day3>()
}

struct Day3;
impl aoc2021::Solution for Day3 {
    const DAY: u8 = 3;
    type Out1 = u32;
    type Out2 = u32;

    /// Tries to be somewhat efficient by keeping a rolling bit count when doing part 2.
    ///
    /// It's probably still terrible.
    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let mut input = input.lines();
        let (first, rest) = (
            input.next().context("need at least 1 line of input")?,
            input,
        );
        let bit_len = first.len();
        let words = iter::once(first)
            .chain(rest)
            .map(|line| u32::from_str_radix(line, 2).context("failed to parse binary"))
            .collect::<Result<Vec<u32>>>()?;

        /// Can count up or down; modifies each counter according to the corresponding
        /// bit in the provided word. At the end, the sign of the counter indicates
        /// which bit was more common in that position.
        ///
        /// The lowest bit in the word is counted at counters[0]. As many bits are counted
        /// as counters provided.
        ///
        /// When counting up, 1 increments the counter and 0 decrements it.
        ///
        /// When counting down, 1 decrements the counter, and 0 increments it.
        fn count_bits(counters: &mut [isize], mut word: u32, up: isize) {
            for count in counters.iter_mut() {
                let bit = word & 0x1;
                let bit = 2 * (bit as isize) - 1; // true: +1, false: -1
                let bit = bit * up;
                *count += bit;
                word >>= 1;
            }
        }

        let bit_counts = words
            .iter()
            .copied()
            .fold(vec![0; bit_len], |mut counts, word| {
                count_bits(&mut counts, word, 1);
                counts
            });

        let (gamma, epsilon) = bit_counts.iter().rev().fold((0, 0), |(g, e), &count| {
            let g = (g << 1) + count.is_positive() as u32;
            let e = (e << 1) + count.is_negative() as u32;
            (g, e)
        });
        let part1 = gamma * epsilon;

        /// Repeatedly filter out words with undesired bits,
        /// starting with the highest bit counted and moving low.
        ///
        /// Returns the final remaining value.
        fn trim_to_final(
            mut words: Vec<u32>,
            mut counters: Vec<isize>,
            choose_desired: impl Fn(isize) -> bool,
        ) -> Result<u32> {
            let mut last = words
                .last()
                .copied()
                .context("cannot trim empty word list")?;
            while words.len() > 1 {
                let hi_count = counters.pop().context("ran out of bits in trim loop")?;
                let desired = choose_desired(hi_count);
                let shift = counters.len();
                let mask = 1u32 << shift;

                last = words.last().copied().unwrap();

                words
                    .drain_filter(|&mut word| desired ^ (word & mask > 0))
                    .for_each(|removed| count_bits(&mut counters, removed, -1));
            }
            Ok(words.get(0).copied().unwrap_or(last))
        }

        let words2 = words.clone();
        let bit_counts2 = bit_counts.clone();
        let o2 = trim_to_final(words2, bit_counts2, |count| !count.is_negative())?;
        let co2 = trim_to_final(words, bit_counts, |count| count.is_negative())?;

        let part2 = o2 * co2;

        Ok((part1, part2))
    }
}

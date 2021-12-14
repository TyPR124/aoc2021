#![feature(array_windows)]

use std::collections::HashMap;

use anyhow::{ensure, Context, Result};
use aoc2021::IterTools;

#[test]
fn test_example() {
    const INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;
    aoc2021::test_solution::<Day14, _, _, _>(INPUT, (1588, 2188189693529));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day14>()
}

struct Day14;
impl aoc2021::Solution for Day14 {
    const DAY: u8 = 14;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let mut lines = input.lines();
        let init = lines.next().context("missing input")?;
        let mut pair_map = HashMap::new();
        for line in lines.skip(1) {
            let (pair, insert) = line.split_once(" -> ").context("failed to parse input")?;
            ensure!(
                pair.len() == 2 && insert.len() == 1,
                "invalid pair or insert lens"
            );
            let pair = [pair.as_bytes()[0], pair.as_bytes()[1]];
            let insert = insert.as_bytes()[0];
            pair_map.insert(pair, insert);
        }
        let mut counts = HashMap::<u8, usize>::new();
        let init: Vec<_> = init.bytes().collect();
        for &s in &init {
            *counts.entry(s).or_default() += 1;
        }
        let mut pcounts = HashMap::<[u8; 2], usize>::new();
        for pair in init.array_windows().copied() {
            *pcounts.entry(pair).or_default() += 1;
        }
        let mut after_steps = |n| {
            for _ in 0..n {
                let mut new = HashMap::new();
                for (pair, count) in &pcounts {
                    let insert = pair_map[pair];
                    *counts.entry(insert).or_default() += count;
                    *new.entry([pair[0], insert]).or_default() += count;
                    *new.entry([insert, pair[1]]).or_default() += count;
                }
                pcounts = new;
            }

            counts
                .values()
                .copied()
                .min_max()
                .map(|(min, max)| max - min)
        };

        let part1 = after_steps(10).context("part1 steps failed")?;
        let part2 = after_steps(30).context("part2 steps failed")?;

        Ok((part1, part2))
    }
}

#![feature(int_abs_diff)]

use anyhow::{Context, Result};

#[test]
fn test_day7() {
    const INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14"#;
    aoc2021::test_solution::<Day7, _, _, _>(INPUT, (37, 168));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day7>()
}

struct Day7;
impl aoc2021::Solution for Day7 {
    const DAY: u8 = 7;
    type Out1 = u32;
    type Out2 = u32;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let best_fuel = |min, max, positions: &[u32], fuel_cost: fn(u32, u32) -> u32| {
            (min..=max)
                .map(|p| positions.iter().map(|&n| fuel_cost(n, p)).sum())
                .try_fold(
                    u32::MAX,
                    |best, new| {
                        if new <= best {
                            Ok(new)
                        } else {
                            Err(best)
                        }
                    },
                )
                .unwrap_or_else(|best| best)
        };
        let p1_fuel_cost = u32::abs_diff;
        let p2_fuel_cost = |a: u32, b: u32| a.abs_diff(b) * (a.abs_diff(b) + 1) / 2;
        let mut min = u32::MAX;
        let mut max = 0;
        let positions: Vec<u32> = input
            .trim()
            .split(',')
            .map(|n| n.parse().context("failed to parse number"))
            .try_fold::<_, _, Result<_>>(vec![], |mut vec, p: Result<u32>| {
                let p = p?;
                min = p.min(min);
                max = p.max(max);
                vec.push(p);
                Ok(vec)
            })?;
        let (min, max) = (min, max);

        let part1 = best_fuel(min, max, &positions, p1_fuel_cost);
        let part2 = best_fuel(min, max, &positions, p2_fuel_cost);
        Ok((part1, part2))
    }
}

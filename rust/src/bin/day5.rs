use std::{cmp::Ordering, collections::HashMap};

use anyhow::{Context, Result};

#[test]
fn test_day5() {
    const INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;
    aoc2021::test_solution::<Day5, _, _, _>(INPUT, (5, 12));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day5>()
}

struct Day5;
impl aoc2021::Solution for Day5 {
    const DAY: u8 = 5;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let lines = input.lines().map::<Result<_, anyhow::Error>, _>(|line| {
            let (x1, line) = line.split_once(',').context("failed to parse line")?;
            let (y1, line) = line.split_once(' ').context("failed to parse line")?;
            let (_, line) = line.split_once(' ').context("failed to parse line")?;
            let (x2, y2) = line.split_once(',').context("failed to parse line")?;
            let x1: u16 = x1.parse().context("failed to parse x1")?;
            let x2: u16 = x2.parse().context("failed to parse x2")?;
            let y1: u16 = y1.parse().context("failed to parse y1")?;
            let y2: u16 = y2.parse().context("failed to parse y2")?;
            Ok([(x1, y1), (x2, y2)])
        });

        #[derive(Copy, Clone, Default)]
        struct Count {
            hv: usize,
            d: usize,
        }

        let mut points = HashMap::<_, Count>::new();

        for line in lines {
            let [(x1, y1), (x2, y2)] = line?;

            match (x1.cmp(&x2), y1.cmp(&y2)) {
                (Ordering::Equal, _) => {
                    let x = x1;
                    let (y1, y2) = (y1.min(y2), y1.max(y2));
                    (y1..=y2).for_each(|y| points.entry((x, y)).or_default().hv += 1)
                }
                (_, Ordering::Equal) => {
                    let y = y1;
                    let (x1, x2) = (x1.min(x2), x1.max(x2));
                    (x1..=x2).for_each(|x| points.entry((x, y)).or_default().hv += 1)
                }
                (dx, dy) if dx == dy => {
                    let (x1, x2) = (x1.min(x2), x1.max(x2));
                    let (y1, y2) = (y1.min(y2), y1.max(y2));
                    (x1..=x2)
                        .zip(y1..=y2)
                        .for_each(|(x, y)| points.entry((x, y)).or_default().d += 1)
                }
                _ => {
                    let (x1, x2) = (x1.min(x2), x1.max(x2));
                    let (y1, y2) = (y1.min(y2), y1.max(y2));
                    (x1..=x2)
                        .rev()
                        .zip(y1..=y2)
                        .for_each(|(x, y)| points.entry((x, y)).or_default().d += 1)
                }
            }
        }

        let (part1, part2) = points.values().fold((0, 0), |(p1, p2), &count| {
            let p1 = p1 + (count.hv > 1) as usize;
            let p2 = p2 + (count.hv + count.d > 1) as usize;
            (p1, p2)
        });

        Ok((part1, part2))
    }
}

#![feature(int_abs_diff)]

use anyhow::{Context, Result};
use aoc2021::IterTools;

#[test]
fn test_example() {
    const INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;
    aoc2021::test_solution::<Day13, _, _, _>(
        INPUT,
        (
            17,
            r#"█████
█   █
█   █
█   █
█████"#,
        ),
    )
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day13>()
}

struct Day13;
impl aoc2021::Solution for Day13 {
    const DAY: u8 = 13;
    type Out1 = usize;
    type Out2 = String;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let lines = &mut input.lines();
        let mut points: Vec<(u32, u32)> = lines
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (x, y) = line.split_once(',')?;
                let x = x.parse().ok()?;
                let y = y.parse().ok()?;
                Some((x, y))
            })
            .collect::<Option<_>>()
            .context("failed to parse input")?;

        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        enum Fold {
            X(u32),
            Y(u32),
        }
        let folds = lines.map(|line| {
            let fold = line.strip_prefix("fold along ")?;
            let (axis, value) = fold.split_once('=')?;
            let value = value.parse().ok()?;
            match axis {
                "x" => Some(Fold::X(value)),
                "y" => Some(Fold::Y(value)),
                _ => None,
            }
        });

        let mut part1 = None;

        for fold in folds {
            let fold = fold.context("failed to parse fold")?;
            let do_fold = |p: u32, at| at - p.abs_diff(at);
            match fold {
                Fold::X(at) => {
                    for (x, _) in &mut points {
                        *x = do_fold(*x, at);
                    }
                }
                Fold::Y(at) => {
                    for (_, y) in &mut points {
                        *y = do_fold(*y, at);
                    }
                }
            }
            points.sort_unstable();
            points.dedup();
            if part1.is_none() {
                part1 = Some(points.len())
            }
        }
        let part1 = part1.context("failed to find part1")?;
        // If we got part1, there must have been at least 1 fold.
        // Therefore, we can assume points are sorted.

        let &(x_min, _) = points.get(0).context("no points left")?;
        let &(x_max, _) = points.last().unwrap();
        let (y_min, y_max) = points
            .iter()
            .map(|(_, y)| *y)
            .min_max()
            .context("less than two points left")?;

        let mut part2 = String::new();

        (y_min..=y_max).for_each(|y| {
            (x_min..=x_max).for_each(|x| {
                let c = if points.contains(&(x, y)) { '█' } else { ' ' };
                part2.push(c);
            });
            part2.push('\n');
        });
        part2.pop();

        Ok((part1, part2))
    }
}

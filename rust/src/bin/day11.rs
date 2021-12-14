use anyhow::{Context, Result};
use aoc2021::{Ascii, Grid, GridIndex, GridSize, Neighbors};

use std::{collections::HashSet, iter};

#[test]
fn test_example() {
    const INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;
    aoc2021::test_solution::<Day11, _, _, _>(INPUT, (1656, 195));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day11>()
}

struct Day11;
impl aoc2021::Solution for Day11 {
    const DAY: u8 = 11;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let mut lines = input.lines();
        let first = lines.next().context("missing input")?;
        let width = first.len();
        let input: Vec<u8> = iter::once(first)
            .chain(lines)
            .flat_map(|line| line.bytes())
            .map(|x| x.to_digit())
            .collect::<Option<_>>()
            .context("invalid input")?;
        let height = input.len() / width;
        let size = GridSize { width, height };
        let mut input = Grid::new(input, size).context("invalid input")?;

        let mut energy_levels = <[HashSet<usize>; 10]>::default();
        input.iter_copied().enumerate().for_each(|(i, level)| {
            energy_levels[level as usize].insert(i);
        });

        let mut part1 = 0;
        let mut part2 = None;

        let mut step = || {
            let indices = (0..input.len()).map(|index| GridIndex { index, size });
            indices.clone().for_each(|i| input[i] += 1);
            let mut flashed = Grid::new(vec![false; input.len()], size).unwrap();
            let mut flash_count = 0;
            let mut changed = true;
            while changed {
                changed = false;
                indices.clone().for_each(|i| {
                    if input[i] > 9 && !flashed[i] {
                        flash_count += 1;
                        flashed[i] = true;
                        changed = true;
                        i.neighbors_iter(Neighbors::All)
                            .for_each(|i| input[i] = input[i].saturating_add(1));
                    }
                });
            }
            input.iter_mut().for_each(|b| {
                if *b > 9 {
                    *b = 0
                }
            });
            flash_count
        };

        for i in 0.. {
            let flashes = step();
            if flashes == size.to_len() && part2.is_none() {
                // index to count
                part2 = Some(i + 1)
            }
            if i < 100 {
                part1 += flashes;
            }
            if i >= 100 && part2.is_some() {
                break;
            }
        }
        let part2 = part2.unwrap();

        Ok((part1, part2))
    }
}

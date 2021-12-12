use std::collections::HashSet;

use anyhow::{Context, Result};
use aoc2021::{Grid, GridIndex, GridSize, Neighbors::Cardinal};

#[test]
fn test_example() {
    const INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;
    aoc2021::test_solution::<Day9, _, _, _>(INPUT, (15, 1134));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day9>()
}

struct Day9;
impl aoc2021::Solution for Day9 {
    const DAY: u8 = 9;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines.get(0).context("missing input")?.len();
        let height = lines.len();

        let input: Vec<u8> = lines
            .into_iter()
            .flat_map(|line| line.bytes())
            .map(|x| x - b'0')
            .collect();
        let input = Grid::new(input, GridSize { width, height })?;

        let is_low_point = |index: GridIndex| {
            let this = input[index];
            index
                .neighbors_iter(Cardinal)
                .all(|neighbor| input[neighbor] > this)
        };

        let basin_size = |low_point: GridIndex| {
            let mut basin = HashSet::new();
            basin.insert(low_point.index);
            let mut filter =
                |&neighbor: &GridIndex| input[neighbor] != 9 && basin.insert(neighbor.index);
            let mut neighbors: Vec<_> = low_point
                .neighbors_iter(Cardinal)
                .filter(&mut filter)
                .collect();
            let mut size = neighbors.len() + 1;
            while let Some(neighbor) = neighbors.pop() {
                neighbors.extend(
                    neighbor
                        .neighbors_iter(Cardinal)
                        .filter(&mut filter)
                        .inspect(|_| size += 1),
                )
            }
            size
        };

        let mut basins = vec![];
        let part1 = (0..input.len())
            .map(|index| GridIndex {
                index,
                size: input.size(),
            })
            .filter_map(|index| {
                is_low_point(index).then(|| {
                    basins.push(basin_size(index));
                    input[index] as usize + 1
                })
            })
            .sum();
        basins.sort_unstable();
        let part2 = basins.into_iter().rev().take(3).product();

        Ok((part1, part2))
    }
}

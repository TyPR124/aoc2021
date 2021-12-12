use anyhow::{bail, Context, Result};

#[test]
fn test_example() {
    const INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
    aoc2021::test_solution::<Day2, _, _, _>(INPUT, (150, 900));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day2>()
}

struct Day2;
impl aoc2021::Solution for Day2 {
    const DAY: u8 = 2;
    type Out1 = i64;
    type Out2 = i64;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let input = input.lines().map::<Result<_>, _>(|line| {
            let (dir, dist) = line.split_once(' ').context("failed to parse line")?;
            let dist = dist.parse::<i64>().context("failed to parse number")?;
            Ok((dir, dist))
        });

        let mut part1_hd = 0;
        let mut part1_vd = 0;
        let mut part2_aim = 0;
        let mut part2_vd = 0;

        for pair in input {
            let (dir, dist) = pair?;
            match dir {
                "forward" => {
                    part1_hd += dist;
                    part2_vd += part2_aim * dist;
                }
                "down" => {
                    part1_vd += dist;
                    part2_aim += dist;
                }
                "up" => {
                    part1_vd -= dist;
                    part2_aim -= dist;
                }
                _ => bail!("invalid direction '{}'", dir),
            }
        }

        Ok((part1_hd * part1_vd, part1_hd * part2_vd))
    }
}

use anyhow::{Context, Result};

#[test]
fn test_example() {
    const INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263"#;
    aoc2021::test_solution::<Day1, _, _, _>(INPUT, (7, 5));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day1>()
}

struct Day1;
impl aoc2021::Solution for Day1 {
    const DAY: u8 = 1;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let mut input = input
            .lines()
            .map(|line| line.parse::<i64>().context("failed to parse number"));

        let mut last3 = input.next().context("need at least 3 numbers")??;
        let mut last2 = input.next().context("need at least 3 numbers")??;
        let mut last1 = input.next().context("need at least 3 numbers")??;

        let mut count1 = (last2 > last3) as usize + (last1 > last2) as usize;
        let mut count3 = 0;

        for n in input {
            let n = n?;
            count1 += (n > last1) as usize;
            count3 += (n > last3) as usize;

            last3 = last2;
            last2 = last1;
            last1 = n;
        }

        Ok((count1, count3))
    }
}

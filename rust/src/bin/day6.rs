use anyhow::{Context, Result};

#[test]
fn test_day6() {
    const INPUT: &str = r#"3,4,3,1,2"#;
    aoc2021::test_solution::<Day6, _, _, _>(INPUT, (5934, 26984457539));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day6>()
}

struct Day6;
impl aoc2021::Solution for Day6 {
    const DAY: u8 = 6;
    type Out1 = u64;
    type Out2 = u64;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let mut timers = [0u64; 9];

        for timer in input
            .trim()
            .split(',')
            .map(|s| s.parse::<u8>().context("failed to parse timer"))
        {
            let timer = timer?;
            *timers
                .get_mut(timer as usize)
                .context("invalid timer start value")? += 1;
        }

        fn step7(timers: [u64; 9]) -> [u64; 9] {
            // 0 -> 0,2
            // 1 -> 1,3
            // 2 -> 2,4
            // 3 -> 3,5
            // 4 -> 4,6
            // 5 -> 5,7
            // 6 -> 6,8
            // 7 -> 0
            // 8 -> 1
            [
                timers[0] + timers[7],
                timers[1] + timers[8],
                timers[2] + timers[0],
                timers[3] + timers[1],
                timers[4] + timers[2],
                timers[5] + timers[3],
                timers[6] + timers[4],
                timers[5],
                timers[6],
            ]
        }
        fn step1(timers: [u64; 9]) -> [u64; 9] {
            [
                timers[1],
                timers[2],
                timers[3],
                timers[4],
                timers[5],
                timers[6],
                timers[7] + timers[0],
                timers[8],
                timers[0],
            ]
        }

        let sevens = 80 / 7;
        let ones = 80 % 7;
        for _ in 0..sevens {
            timers = step7(timers)
        }
        for _ in 0..ones {
            timers = step1(timers)
        }
        let part1 = timers.iter().copied().sum();

        let sevens = (256 - 80) / 7;
        let ones = (256 - 80) % 7;
        for _ in 0..sevens {
            timers = step7(timers)
        }
        for _ in 0..ones {
            timers = step1(timers)
        }
        let part2 = timers.iter().copied().sum();

        Ok((part1, part2))
    }
}

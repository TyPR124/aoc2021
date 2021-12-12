use anyhow::{Context, Result};

#[test]
fn test_example() {
    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
    aoc2021::test_solution::<Day8, _, _, _>(INPUT, (26, 61229));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day8>()
}

struct Day8;
impl aoc2021::Solution for Day8 {
    const DAY: u8 = 8;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        fn bits_from_segments(segments: &str) -> u8 {
            segments
                .bytes()
                .fold(0, |bits, segment| bits | (1 << (segment - b'a')))
        }
        let input: Vec<(Vec<u8>, Vec<u8>)> = input
            .lines()
            .map(|line| {
                let (pattern, code) = line.split_once('|').context("failed to parse line")?;
                let pattern: Vec<u8> = pattern
                    .split_ascii_whitespace()
                    .map(bits_from_segments)
                    .collect();
                let code: Vec<u8> = code
                    .split_ascii_whitespace()
                    .map(bits_from_segments)
                    .collect();
                Ok((pattern, code))
            })
            .collect::<Result<_>>()?;

        fn solve_pattern(pattern: &[u8]) -> Option<[u8; 10]> {
            let &one = pattern.iter().find(|&&p| p.count_ones() == 2)?;
            let &seven = pattern.iter().find(|&&p| p.count_ones() == 3)?;
            let &four = pattern.iter().find(|&&p| p.count_ones() == 4)?;
            let &eight = pattern.iter().find(|&&p| p.count_ones() == 7)?;
            let &three = pattern
                .iter()
                .find(|&&p| p.count_ones() == 5 && (p & one).count_ones() == 2)?;
            let &two = pattern
                .iter()
                .find(|&&p| p.count_ones() == 5 && (p & four).count_ones() == 2)?;
            let &five = pattern
                .iter()
                .find(|&&p| p != three && p.count_ones() == 5 && (p & four).count_ones() == 3)?;
            let &six = pattern
                .iter()
                .find(|&&p| p.count_ones() == 6 && (p & one).count_ones() == 1)?;
            let &zero = pattern
                .iter()
                .find(|&&p| p != six && p.count_ones() == 6 && (p & five).count_ones() == 4)?;
            let &nine = pattern
                .iter()
                .find(|&&p| p != zero && p != six && p.count_ones() == 6)?;

            Some([zero, one, two, three, four, five, six, seven, eight, nine])
        }

        let mut part1 = 0;
        let mut part2 = 0;

        for (pattern, code) in input {
            let pattern = solve_pattern(&pattern).context("bad pattern")?;
            let mut digits = 0;
            for bits in code {
                let digit = pattern
                    .iter()
                    .enumerate()
                    .flat_map(|(i, &p)| (bits == p).then(|| i))
                    .next()
                    .context("failed to match digit")?;
                if digit == 1 || digit == 4 || digit == 7 || digit == 8 {
                    part1 += 1;
                }
                digits = digits * 10 + digit;
            }
            part2 += digits;
        }

        Ok((part1, part2))
    }
}

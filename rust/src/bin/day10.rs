use anyhow::{bail, Context, Result};

#[test]
fn test_example() {
    const INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;
    aoc2021::test_solution::<Day10, _, _, _>(INPUT, (26397, 288957));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day10>()
}

struct Day10;
impl aoc2021::Solution for Day10 {
    const DAY: u8 = 10;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let mut part1 = 0;
        let mut p2_scores = Vec::<usize>::new();
        'lines: for line in input.lines() {
            let mut stack = vec![];
            for b in line.bytes() {
                match b {
                    b'(' | b'[' | b'{' | b'<' => stack.push(b + 2 - (b == b'(') as u8),
                    _ => {
                        let want = stack.pop().context("ran out of open brackets")?;
                        if b != want {
                            let points = match b {
                                b')' => 3,
                                b']' => 57,
                                b'}' => 1197,
                                b'>' => 25137,
                                _ => bail!("invalid input"),
                            };
                            part1 += points;
                            continue 'lines;
                        }
                    }
                }
            }
            let p2_score = stack
                .into_iter()
                .rev()
                .map(|b| match b {
                    b')' => 1,
                    b']' => 2,
                    b'}' => 3,
                    b'>' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |score, points| score * 5 + points);
            p2_scores.push(p2_score);
        }

        p2_scores.sort_unstable();
        let &part2 = p2_scores
            .get(p2_scores.len() / 2)
            .context("no scores found")?;

        Ok((part1, part2))
    }
}

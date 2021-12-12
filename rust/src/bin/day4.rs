use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

#[test]
fn test_example() {
    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
    aoc2021::test_solution::<Day4, _, _, _>(INPUT, (4512, 1924));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day4>()
}

struct Day4;
impl aoc2021::Solution for Day4 {
    const DAY: u8 = 4;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let mut lines = input.lines();

        let drawn = lines.next().context("missing input")?;
        let drawn = drawn
            .split(',')
            .map(|s| s.parse::<u8>().context("failed to parse drawn number"));

        let mut board_numbers = lines
            .flat_map(|line| line.split_ascii_whitespace())
            .map(|s| s.parse::<u8>().context("failed to parse board number"))
            .peekable();

        /*
               5  6  7  8  9
            0 00 01 02 03 04
            1 05 06 07 08 09
            2 10 11 12 13 14
            3 15 16 17 18 19
            4 20 21 22 23 24
        */
        type Board = [u8; 10]; // 10 winning lines
        type Positions = Vec<(usize, usize)>; // (board, line)
        type PositionsMap = HashMap<u8, Positions>;

        fn score_board(
            positions: &PositionsMap,
            winning_board: usize,
            winning_number: u8,
        ) -> usize {
            let sum: usize = positions
                .iter()
                .flat_map(|(&n, spots)| {
                    spots
                        .iter()
                        .filter_map(move |&(board, _)| (board == winning_board).then(|| n as usize))
                        .take(1)
                })
                .sum();

            sum * winning_number as usize
        }

        let mut board_count = 0;
        let mut positions = PositionsMap::default();
        while board_numbers.peek().is_some() {
            for (i, n) in (&mut board_numbers).take(25).enumerate() {
                let n = n?;
                let row = i / 5;
                let col = i % 5;

                positions
                    .entry(n)
                    .or_default()
                    .extend_from_slice(&[(board_count, row), (board_count, col + 5)]);
            }
            board_count += 1;
        }
        let board_count = board_count;

        let mut boards = vec![Board::default(); board_count];
        let mut boards_won = HashSet::new();

        let mut part1 = None;
        let mut part2 = None;

        'drawing: for n in drawn {
            let n = n?;
            if let Some(spots) = positions.remove(&n) {
                for (board, line) in spots {
                    let marks = &mut boards[board][line];
                    *marks += 1;
                    if *marks == 5 && boards_won.insert(board) {
                        if part1.is_none() {
                            part1 = Some(score_board(&positions, board, n));
                        }
                        if boards_won.len() == board_count {
                            part2 = Some(score_board(&positions, board, n));
                            break 'drawing;
                        }
                    }
                }
            }
        }
        let part1 = part1.context("part1 not found")?;
        let part2 = part2.context("part2 not found")?;

        Ok((part1, part2))
    }
}

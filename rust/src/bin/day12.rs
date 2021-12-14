use std::fmt;

use anyhow::{ensure, Context, Result};
use aoc2021::Ascii;
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[test]
fn test_example1() {
    const INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;
    aoc2021::test_solution::<Day12, _, _, _>(INPUT, (10, 36));
}

#[test]
fn test_example2() {
    const INPUT: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;
    aoc2021::test_solution::<Day12, _, _, _>(INPUT, (19, 103));
}

#[test]
fn test_example3() {
    const INPUT: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;
    aoc2021::test_solution::<Day12, _, _, _>(INPUT, (226, 3509));
}

fn main() -> Result<()> {
    aoc2021::run_solution::<Day12>()
}

struct Day12;
impl aoc2021::Solution for Day12 {
    const DAY: u8 = 12;
    type Out1 = usize;
    type Out2 = usize;

    fn solve(input: String) -> Result<(Self::Out1, Self::Out2)> {
        let start = CaveId::from_str("start").unwrap();
        let end = CaveId::from_str("end").unwrap();

        let mut paths = HashMap::<CaveId, HashSet<CaveId>>::default();
        input.lines().try_for_each::<_, Result<_>>(|line| {
            let (a, b) = line.split_once('-').context("missing '-' in input")?;
            let a = CaveId::from_str(a)?;
            let b = CaveId::from_str(b)?;
            paths.entry(a).or_default().insert(b);
            paths.entry(b).or_default().insert(a);
            Ok(())
        })?;

        let mut part1 = 0;
        let mut part2 = 0;

        let mut possible = vec![(start, HashSet::default(), false)];

        while let Some((position, mut smalls_visited, have_second_small)) = possible.pop() {
            for &neighbor in &paths[&position] {
                if neighbor == start {
                    continue;
                }
                if neighbor == end {
                    if !have_second_small {
                        part1 += 1;
                    }
                    part2 += 1;
                    continue;
                }

                let is_small_and_new = neighbor.is_small() && smalls_visited.insert(neighbor);
                let is_second_small =
                    neighbor.is_small() && !is_small_and_new && !have_second_small;
                if neighbor.is_big() || is_small_and_new || is_second_small {
                    possible.push((
                        neighbor,
                        smalls_visited.clone(),
                        have_second_small || is_second_small,
                    ))
                }
                if is_small_and_new {
                    smalls_visited.remove(&neighbor);
                }
            }
        }

        Ok((part1, part2))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct CaveId {
    id: u64,
}

impl CaveId {
    // highest bit indicates big cave
    const BIG_BIT: u64 = i64::MIN as u64;
    pub fn from_str(s: &str) -> Result<Self> {
        ensure!(
            (1..=13).contains(&s.len()),
            "cave id must have length between 1 and 13 chars"
        );
        let is_small = s.as_bytes()[0].is_ascii_lowercase();
        let alphabet_index = if is_small {
            u8::to_lowercase_index
        } else {
            u8::to_uppercase_index
        };

        let id = s.bytes().try_fold(0, |id, b| {
            alphabet_index(b)
                .context("invalid char in cave id")
                .map(|b| id * 26 + b as u64)
        })?;
        let id = if is_small { id } else { id | Self::BIG_BIT };
        Ok(Self { id })
    }
    pub fn is_small(self) -> bool {
        0 == self.id & Self::BIG_BIT
    }
    pub fn is_big(self) -> bool {
        !self.is_small()
    }
}
// Unused, but could be useful
impl fmt::Display for CaveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut id = self.id;
        let base = if self.is_small() { b'a' } else { b'A' };
        if self.is_big() {
            id &= !Self::BIG_BIT;
        }

        let mut bytes = [0u8; 13];
        for b in bytes.iter_mut().rev() {
            *b = base + (id % 26) as u8;
            id /= 26;
            if id == 0 {
                break;
            }
        }
        for b in bytes.into_iter().skip_while(|&b| b == 0) {
            use fmt::Write;
            f.write_char(b as char)?;
        }
        Ok(())
    }
}
// Unused, but could be useful
impl fmt::Debug for CaveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:X} ({})", self.id, self))
    }
}

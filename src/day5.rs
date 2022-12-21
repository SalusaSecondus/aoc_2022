use std::str::FromStr;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

type Input = (Vec<Vec<char>>, Vec<Move>);
type Output = String;

#[derive(Debug, Clone, Copy)]
struct Move {
    count: usize,
    source: usize,
    dest: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
        };
        let mat = RE.captures(s).context("no match")?;
        let count = mat.get(1).context("No count")?.as_str().parse()?;
        let source = mat.get(2).context("No source")?.as_str().parse()?;
        let dest = mat.get(3).context("No dest")?.as_str().parse()?;
        Ok(Self {
            count,
            source,
            dest,
        })
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Result<Input> {
    let mut lines = input.lines();
    let mut stacks = vec![];
    stacks.push(vec![]);
    let mut moves = vec![];

    loop {
        let l = lines.next().unwrap();
        if l.contains('1') {
            lines.next();
            break;
        }
        let chars: Vec<char> = l.chars().collect();
        for (idx, chunk) in chars.chunks(4).enumerate() {
            // println!("Chunk {} is {:?}", idx, chunk);
            if stacks.len() <= idx + 1 {
                stacks.push(vec![]);
            }
            if chunk[1] != ' ' {
                stacks[idx + 1].push(chunk[1]);
            }
        }
    }

    for s in stacks.iter_mut() {
        s.reverse();
    }

    // println!("{:?}", stacks);
    // Now, read the rules
    for l in lines {
        moves.push(l.parse()?);
    }
    Ok((stacks, moves))
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut stacks = input.0.clone();

    for m in &input.1 {
        // println!("{:?}", m);

        for _ in 0..m.count {
            let c = stacks[m.source].pop().context("Underflow")?;
            stacks[m.dest].push(c);
        }
    }
    let mut result = String::new();
    for s in &stacks {
        result += &s.last().map(|c| c.to_string()).unwrap_or_default();
    }
    Ok(result)
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> Result<Output> {
    let mut stacks = input.0.clone();

    for m in &input.1 {
        // println!("{:?}", m);

        let mut current = vec![];
        for _ in 0..m.count {
            let c = stacks[m.source].pop().context("Underflow")?;
            current.push(c);
        }
        current.reverse();
        stacks[m.dest].extend(current);
    }
    let mut result = String::new();
    for s in &stacks {
        result += &s.last().map(|c| c.to_string()).unwrap_or_default();
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, "CMZ");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, "MCD");
        Ok(())
    }
}

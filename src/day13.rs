use std::{cmp::Ordering, fmt::Display, str::FromStr};

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::{self, complete::i32},
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

type Input = Vec<(Packet, Packet)>;
type Output = i32;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match nom_packet_list(s) {
            Ok((_, i)) => Ok(i),
            Err(e) => bail!("Error parsing {}", e),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Packet::Number(left) => match other {
                Packet::Number(right) => left.partial_cmp(right),
                Packet::List(right) => vec![Packet::Number(*left)].partial_cmp(right),
            },
            Packet::List(left) => match other {
                Packet::Number(right) => left.partial_cmp(&vec![Packet::Number(*right)]),
                Packet::List(right) => left.partial_cmp(right),
            },
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(n) => write!(f, "{}", n),
            Packet::List(v) => write!(f, "[{}]", v.iter().join(",")),
        }
    }
}

fn nom_packet_list(s: &str) -> IResult<&str, Packet> {
    let num_parser = map(i32, Packet::Number);
    map(
        delimited(
            character::complete::char('['),
            separated_list0(
                character::complete::char(','),
                alt((num_parser, nom_packet_list)),
            ),
            character::complete::char(']'),
        ),
        Packet::List,
    )(s)
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = vec![];
    for p in input.split("\n\n") {
        let e = p.split_once('\n').context("Misformed")?;

        result.push((e.0.parse()?, e.1.parse()?))
    }
    Ok(result)
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut result: i32 = 0;
    for (idx, pair) in input.iter().enumerate() {
        if pair.0.cmp(&pair.1) == Ordering::Less {
            // princtln!("Adding idx: {}", idx + 1);
            result += idx as i32 + 1;
        }
    }
    Ok(result)
}

#[aoc(day13, part2)]
fn part2(input: &Input) -> Result<Output> {
    let indicator1: Packet = "[[2]]".parse()?;
    let indicator2: Packet = "[[6]]".parse()?;
    let mut list = input
        .iter()
        .flat_map(|p| vec![p.0.clone(), p.1.clone()])
        .collect_vec();
    list.push(indicator1.clone());
    list.push(indicator2.clone());
    list.sort_unstable();

    let mut result = 1;
    for (idx, packet) in list.iter().enumerate() {
        // println!("{}:\t {}", idx, packet);
        if packet == &indicator1 || packet == &indicator2 {
            result *= idx as i32 + 1;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        for p in &input {
            println!("{}\n{}\n\t{:?}\n", p.0, p.1, p.0.cmp(&p.1));
        }
        assert_eq!(part1(&input)?, 13);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 140);
        Ok(())
    }
}

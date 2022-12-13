use std::{cmp::Ordering, fmt::Display, str::FromStr};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<(Packet, Packet)>;
type Output = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Close,
    Number(i32),
}

fn tokenize(s: &str) -> Result<Vec<Token>> {
    let mut result = vec![];

    let mut working = String::new();
    for c in s.chars() {
        // println!("Tokenizing: {}", c);
        match c {
            '[' => result.push(Token::Open),
            ']' => {
                if !working.is_empty() {
                    result.push(Token::Number(working.parse()?));
                    working.clear();
                };
                result.push(Token::Close);
            }
            ',' => {
                if !working.is_empty() {
                    result.push(Token::Number(working.parse()?));
                    working.clear();
                };
            }
            _ => working += &c.to_string(),
        }
    }

    Ok(result)
}

fn packetize(tokens: &mut impl Iterator<Item = Token>) -> Option<Packet> {
    if let Some(next) = tokens.next() {
        if let Token::Number(n) = next {
            Some(Packet::Number(n))
        } else if Token::Close == next {
            None
        } else {
            // Open
            let mut list = vec![];

            while let Some(p) = packetize(tokens) {
                list.push(p);
            }
            Some(Packet::List(list))
        }
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = tokenize(s)?;
        let result = packetize(&mut tokens.iter().copied());
        result.context("No packet found")
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Packet::Number(s) = self {
            if let Packet::Number(o) = other {
                Some(s.cmp(o))
            } else {
                Packet::List(vec![Packet::Number(*s)]).partial_cmp(other)
            }
        } else {
            if let Packet::Number(n) = other {
                return self.partial_cmp(&Packet::List(vec![Packet::Number(*n)]));
            }
            let left = match self {
                Packet::Number(_) => panic!("unreachable1"),
                Packet::List(v) => v,
            };
            let right = match other {
                Packet::Number(_) => panic!("unreachable2"),
                Packet::List(v) => v,
            };
            let mut left = left.iter();
            let mut right = right.iter();
            loop {
                let n_l = left.next();
                let n_r = right.next();

                if n_l.is_none() && n_r.is_none() {
                    return Some(Ordering::Equal);
                } else if n_l.is_none() {
                    return Some(Ordering::Less);
                } else if n_r.is_none() {
                    return Some(Ordering::Greater);
                } else {
                    #[allow(clippy::unnecessary_unwrap)]
                    let sub_ordering = n_l.unwrap().partial_cmp(n_r.unwrap()).unwrap();
                    if sub_ordering != Ordering::Equal {
                        return Some(sub_ordering);
                    }
                }
            }
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

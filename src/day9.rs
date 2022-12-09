use std::{collections::HashSet, str::FromStr};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use strum::EnumString;

type Input = Vec<Command>;
type Output = usize;

type Coord = (i32, i32);

fn offset(head: &Coord, tail: &Coord) -> Coord {
    (head.0 - tail.0, head.1 - tail.1)
}

fn adjacent(head: &Coord, tail: &Coord) -> bool {
    let offset = offset(head, tail);
    offset.0.abs() <= 1 && offset.1.abs() <= 1
}

fn step(coord: &Coord, dir: Direction) -> Coord {
    match dir {
        Direction::U => (coord.0, coord.1 + 1),
        Direction::D => (coord.0, coord.1 - 1),
        Direction::L => (coord.0 - 1, coord.1),
        Direction::R => (coord.0 + 1, coord.1),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Command {
    dist: u32,
    dir: Direction,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').context("No space")?;
        let dir = dir.parse()?;
        let dist = dist.parse()?;
        let result = Command { dir, dist };
        Ok(result)
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Result<Input> {
    let result: Vec<Command> = input
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<Vec<Command>>>()?;
    Ok(result)
}

#[aoc(day9, part1)]
#[allow(clippy::comparison_chain)]
fn part1(input: &Input) -> Result<Output> {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert(tail);

    for cmd in input {
        for _ in 0..cmd.dist {
            head = step(&head, cmd.dir);
            if !adjacent(&head, &tail) {
                let offset = offset(&head, &tail);
                if offset.0 < 0 {
                    tail = step(&tail, Direction::L);
                } else if offset.0 > 0 {
                    tail = step(&tail, Direction::R);
                }
                if offset.1 < 0 {
                    tail = step(&tail, Direction::D);
                } else if offset.1 > 0 {
                    tail = step(&tail, Direction::U);
                }

                visited.insert(tail);
            }
        }
    }
    Ok(visited.len())
}

#[aoc(day9, part2)]
#[allow(clippy::comparison_chain)]
fn part2(input: &Input) -> Result<Output> {
    let mut rope = vec![(0, 0); 10];
    let mut visited: HashSet<Coord> = HashSet::new();
    visited.insert((0, 0));

    for cmd in input {
        for _ in 0..cmd.dist {
            rope[0] = step(&rope[0], cmd.dir);
            for idx in 1..rope.len() {
                if !adjacent(&rope[idx - 1], &rope[idx]) {
                    let offset = offset(&rope[idx - 1], &rope[idx]);
                    if offset.0 < 0 {
                        rope[idx] = step(&rope[idx], Direction::L);
                    } else if offset.0 > 0 {
                        rope[idx] = step(&rope[idx], Direction::R);
                    }
                    if offset.1 < 0 {
                        rope[idx] = step(&rope[idx], Direction::D);
                    } else if offset.1 > 0 {
                        rope[idx] = step(&rope[idx], Direction::U);
                    }
                }
            }
            visited.insert(rope[9]);
        }
    }
    Ok(visited.len())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    const INPUT2: &str = "R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 13);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 1);
        let input = input_generator(INPUT2)?;
        assert_eq!(part2(&input)?, 36);
        Ok(())
    }
}

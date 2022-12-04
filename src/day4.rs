use std::ops::RangeInclusive;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>;
type Output = i32;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = vec![];

    for l in input.lines() {
        let (elf1, elf2) = l.split_once(',').context("No comma")?;
        result.push((assignments_to_range(elf1)?, assignments_to_range(elf2)?));
    }

    Ok(result)
}

fn assignments_to_range(assignments: &str) -> Result<RangeInclusive<i32>> {
    let (start, end) = assignments.split_once('-').context("No dash")?;
    let start: i32 = start.parse()?;
    let end: i32 = end.parse()?;
    Ok(RangeInclusive::new(start, end))
}

fn contains(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    r1.start() <= r2.start() && r1.end() >= r2.end()
}

fn overlaps(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut result = 0;
    for pair in input {
        if contains(&pair.0, &pair.1) || contains(&pair.1, &pair.0) {
            result += 1;
        }
    }
    Ok(result)
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> Result<Output> {
    let mut result = 0;
    for pair in input {
        if overlaps(&pair.0, &pair.1) || contains(&pair.1, &pair.0) {
            result += 1;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 2);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 4);
        Ok(())
    }
}

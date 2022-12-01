use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<String>;
type Output = i32;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Result<Input> {
    Ok(input.lines().map(|s| s.to_owned()).collect())
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> Result<Output> {
    todo!("day 2 part 1")
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> Result<Output> {
    todo!("day 2 part 1")
}

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<Vec<i32>>;
type Output = i32;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut lines = input.lines();
    loop {
        let l = lines.next();
        if l.is_none() {
            return Ok(result);
        }
        let mut l = l.unwrap();
        let mut elf = vec![];
        while !l.is_empty() {
            elf.push(l.parse()?);
            l = lines.next().unwrap_or_default();
        }
        result.push(elf);
    }
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> Result<Output> {
    let m = input
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .context("No data?")?;

    Ok(m)
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> Result<Output> {
    let result = input
        .iter()
        .map(|elf| elf.iter().sum())
        .sorted_unstable_by(|a: &i32, b: &i32| b.cmp(a))
        .take(3)
        .sum();

    Ok(result)
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 24000);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 45000);
        Ok(())
    }
}
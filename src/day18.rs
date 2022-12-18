use std::collections::HashSet;

use anyhow::{Result, Context};
use aoc_runner_derive::{aoc_generator, aoc};

type Coord = (i16, i16, i16);
type Input = HashSet<Coord>;
type Output = i32;

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = HashSet::new();
    for l in input.lines() {
        let mut parts = l.split(',');
        let a = parts.next().context("No part")?.parse()?;
        let b = parts.next().context("No part")?.parse()?;
        let c = parts.next().context("No part")?.parse()?;
        result.insert((a, b, c));
    }
    Ok(result) 
}

#[aoc(day18, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut border = input.len() * 6;;
    for c in input {
        if input.contains(&(c.0 - 1, c.1, c.2)) {
            border -= 1;
        }
        if input.contains(&(c.0 + 1, c.1, c.2)) {
            border -= 1;
        }
        if input.contains(&(c.0, c.1 - 1, c.2)) {
            border -= 1;
        }
        if input.contains(&(c.0, c.1 + 1, c.2)) {
            border -= 1;
        }
        if input.contains(&(c.0, c.1, c.2 - 1)) {
            border -= 1;
        }
        if input.contains(&(c.0, c.1, c.2 + 1)) {
            border -= 1;
        }
    }

    Ok(border as i32)
}

#[aoc(day18, part2)]
fn part2(input: &Input) -> Result<Output> {
    todo!("Write me")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;

        assert_eq!(part1(&input)?, 64);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 58);
        Ok(())
    }
}
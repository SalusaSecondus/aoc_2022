use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = String;
type Output = usize;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Result<Input> {
    Ok(input.to_owned())
}

fn find_start(seq: &str, window_size: usize) -> Option<usize> {
    let seq: Vec<char> = seq.chars().collect();
    for w in seq.windows(window_size).enumerate() {
        let unique = w.1.iter().unique().count();
        if unique == window_size {
            return Some(w.0 + window_size);
        }
    }

    None
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> Result<Output> {
    find_start(input, 4).context("No start")
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> Result<Output> {
    find_start(input, 14).context("No start")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 7);
        assert_eq!(part1(&input_generator(INPUT2)?)?, 5);
        assert_eq!(part1(&input_generator(INPUT3)?)?, 6);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 19);
        Ok(())
    }
}

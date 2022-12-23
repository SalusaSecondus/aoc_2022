use anyhow::Result;
use aoc_runner_derive::{aoc_generator, aoc};

type Input = Vec<String>;
type Output = i32;

#[aoc_generator(dayN)]
fn input_generator(input: &str) -> Result<Input> {
    todo!("Write me")
}

#[aoc(dayN, part1)]
fn part1(input: &Input) -> Result<Output> {
    todo!("Write me")
}

#[aoc(dayN, part1)]
fn part2(input: &Input) -> Result<Output> {
    todo!("Write me")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "input";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        println!("{:?}", input);

        assert_eq!(part1(&input)?, 152);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 301);
        Ok(())
    }
}

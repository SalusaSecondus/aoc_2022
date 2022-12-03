use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(Move, Move)>;
type Output = i32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn move_value(action: Move) -> i32 {
    match action {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn result_score(me: Move, other: Move) -> i32 {
    if me == other {
        3
    } else if (me == Move::Rock && other == Move::Scissors)
        || (me == Move::Paper && other == Move::Rock)
        || (me == Move::Scissors && other == Move::Paper)
    {
        6
    } else {
        0
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = vec![];
    for l in input.lines() {
        let mut parts = l.split_whitespace();
        let opponent = parts.next().context("No opponent move")?;
        let me = parts.next().context("No my move")?;

        let opponent = match opponent {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => bail!("Unexpected value"),
        };
        let me = match me {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => bail!("Unexpected value"),
        };
        result.push((opponent, me));
    }
    Ok(result)
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut result = 0;
    for game in input {
        // print!("{:?}", game);
        result += result_score(game.1, game.0) + move_value(game.1);
    }
    Ok(result)
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> Result<Output> {
    let mut result = 0;
    for game in input {
        let other = game.0;
        let me = match game.1 {
            Move::Rock => match other {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Move::Paper => other,
            Move::Scissors => match other {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
        };
        result += result_score(me, other) + move_value(me);
        // println!("{:?} {:?} {}", other, me, result);
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 15);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 12);
        Ok(())
    }
}

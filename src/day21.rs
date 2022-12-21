use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = HashMap<String, MonkeyAction>;
type Output = i64;

#[derive(Clone, PartialEq, Eq, Debug)]
enum MonkeyAction {
    Number(i64),
    Operation(String, String, String),
}

impl MonkeyAction {
    fn apply(&self, a: i64, b: i64) -> Result<i64> {
        if let MonkeyAction::Operation(_, op, _) = self {
            Ok(match op.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => bail!("Unknown operation: {}", op),
            })
        } else {
            bail!("Only valid for operation");
        }
    }
}

impl FromStr for MonkeyAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect_vec();
        if parts.len() == 1 {
            Ok(MonkeyAction::Number(parts[0].parse()?))
        } else {
            Ok(MonkeyAction::Operation(
                parts[0].to_owned(),
                parts[1].to_owned(),
                parts[2].to_owned(),
            ))
        }
    }
}

fn monkey_value(
    name: &str,
    monkeys: &HashMap<String, MonkeyAction>,
    cache: &mut HashMap<String, i64>,
) -> Result<i64> {
    if let Some(result) = cache.get(name) {
        return Ok(*result);
    }
    let monkey_action = monkeys.get(name).context("Bad monkey name")?;
    let result = match monkey_action {
        MonkeyAction::Number(num) => *num,
        MonkeyAction::Operation(a, _, b) => monkey_action.apply(
            monkey_value(a, monkeys, cache)?,
            monkey_value(b, monkeys, cache)?,
        )?,
    };
    cache.insert(name.to_owned(), result);
    Ok(result)
}

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = HashMap::new();
    for l in input.lines() {
        let parts = l.trim().split_once(": ").context("No name")?;
        let name = parts.0;
        let action = parts.1.parse()?;
        result.insert(name.to_owned(), action);
    }

    Ok(result)
}

#[aoc(day21, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut cache = HashMap::new();
    monkey_value("root", input, &mut cache)
}

fn find_human(
    name: &str,
    target_value: i64,
    monkeys: &HashMap<String, MonkeyAction>,
    cache: &mut HashMap<String, i64>,
) -> Result<i64> {
    if name == "humn" {
        return Ok(target_value);
    }
    if let MonkeyAction::Operation(left, op, right) = monkeys.get(name).context("Bad monkey")? {
        let mut left_cache = cache.clone();
        let left_value = monkey_value(left, monkeys, &mut left_cache)?;
        // We're going to assume that the human is only ever on one side
        if !left_cache.contains_key("humn") {
            cache.extend(left_cache);

            let target_value = match op.as_str() {
                "+" => target_value - left_value,
                "-" => left_value - target_value,
                "/" => left_value / target_value,
                "*" => target_value / left_value,
                _ => bail!("Unsupported "),
            };
            find_human(right, target_value, monkeys, cache)
        } else {
            // Human is on the left
            let right_value = monkey_value(right, monkeys, cache)?;

            let target_value = match op.as_str() {
                "+" => target_value - right_value,
                "-" => target_value + right_value,
                "/" => target_value * right_value,
                "*" => target_value / right_value,
                _ => bail!("Unsupported "),
            };
            find_human(left, target_value, monkeys, cache)
        }
    } else {
        bail!("Monkey action must be operation");
    }
}

#[aoc(day21, part2)]
fn part2(input: &Input) -> Result<Output> {
    // Human is on the left only for both test and real.
    let mut cache = HashMap::new();
    if let MonkeyAction::Operation(a, _, b) = input.get("root").context("missing root")? {
        let target_value = monkey_value(b, input, &mut cache)?;
        find_human(a, target_value, input, &mut cache)
    } else {
        bail!("Root monkey of wrong type");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        println!("{:?}", input);
        // print_map(&input);
        // while !drop_sand(&mut input) {
        //     print_map(&input);
        // }

        assert_eq!(part1(&input)?, 152);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        // for idx in 0..95 {
        //     drop_sand2(&mut input);
        //     println!("Cycle {}:", idx);
        //     print_map(&input);
        // }
        // todo!();
        assert_eq!(part2(&input)?, 301);
        Ok(())
    }
}

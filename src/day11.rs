use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

use anyhow::{ensure, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num_integer::Integer;
use salusa_aoc::SalusaAocIter;

type Input = Vec<Monkey>;
type Output = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Multiply(i64),
    Add(i64),
    Square,
}

impl Operation {
    fn apply(&self, worry: i64) -> i64 {
        match self {
            Operation::Multiply(other) => worry * other,
            Operation::Add(other) => worry + other,
            Operation::Square => worry * worry,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    #[allow(dead_code)]
    id: usize,
    items: VecDeque<i64>,
    operation: Operation,
    test: i64,
    dest_true: usize,
    dest_false: usize,
    inspections: u64,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().context("Too few lines")?;
        let starting = lines.next().context("Too few lines")?;
        let operation = lines.next().context("Too few lines")?;
        let test = lines.next().context("Too few lines")?;
        let dest_true = lines.next().context("Too few lines")?;
        let dest_false = lines.next().context("Too few lines")?;
        ensure!(lines.next().is_none(), "Too many lines");

        let id = header
            .strip_prefix("Monkey ")
            .and_then(|h| h.strip_suffix(':'))
            .context("Bad header")?
            .parse()?;
        let items: std::result::Result<VecDeque<i64>, ParseIntError> = starting
            .strip_prefix("  Starting items: ")
            .context("Bad items")?
            .split(", ")
            .map(|i| i.parse::<i64>())
            .collect();
        let items = items?;
        let operation = operation
            .strip_prefix("  Operation: new = old ")
            .context("Bad operation")?;
        let parts = operation.split_once(' ').context("Bad operation")?;
        let operation = if parts.0 == "+" {
            Operation::Add(parts.1.parse()?)
        } else if parts.1 == "old" {
            Operation::Square
        } else {
            Operation::Multiply(parts.1.parse()?)
        };

        let test = test
            .strip_prefix("  Test: divisible by ")
            .context("Bad test")?
            .parse()?;
        let dest_true = dest_true
            .strip_prefix("    If true: throw to monkey ")
            .context("Bad dest")?
            .parse()?;
        let dest_false = dest_false
            .strip_prefix("    If false: throw to monkey ")
            .context("Bad dest")?
            .parse()?;

        Ok(Self {
            id,
            items,
            operation,
            test,
            dest_true,
            dest_false,
            inspections: 0,
        })
    }
}

fn action(monkeys: &mut [Monkey], index: usize, verbose: bool) -> Result<()> {
    if verbose {
        println!("Monkey {}:", index);
    }
    while !monkeys[index].items.is_empty() {
        let current = monkeys.get_mut(index).context("Bad index")?;
        current.inspections += 1;
        let mut item = current.items.pop_front().unwrap();
        if verbose {
            println!("  Monkey inspects an item with a worry level of {}.", item);
        }
        item = current.operation.apply(item);
        if verbose {
            println!("    Worry level changes to {}.", item);
        }
        item /= 3;
        if verbose {
            println!(
                "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                item
            );
        }

        let test_result = (item % current.test) == 0;
        if verbose {
            println!(
                "    Current worry divisible by {}? {}",
                current.test, test_result
            );
        }
        let dest_monkey = if test_result {
            current.dest_true
        } else {
            current.dest_false
        };
        if verbose {
            println!(
                "    Item with worry {} is thrown to monkey {}.",
                item, dest_monkey
            );
        }
        let current = monkeys.get_mut(dest_monkey).context("Invalid dest")?;
        current.items.push_back(item);
    }
    Ok(())
}

fn action2(monkeys: &mut [Monkey], index: usize, reduction: i64, verbose: bool) -> Result<()> {
    if verbose {
        println!("Monkey {}:", index);
    }
    while !monkeys[index].items.is_empty() {
        let current = monkeys.get_mut(index).context("Bad index")?;
        current.inspections += 1;
        let mut item = current.items.pop_front().unwrap();
        if verbose {
            println!("  Monkey inspects an item with a worry level of {}.", item);
        }
        item = current.operation.apply(item);
        if verbose {
            println!("    Worry level changes to {}.", item);
        }

        item %= reduction;
        if verbose {
            println!(
                "    Monkey gets bored with item. Worry level is reduced to {}.",
                item
            );
        }

        let test_result = (item % current.test) == 0;
        if verbose {
            println!(
                "    Current worry divisible by {}? {}",
                current.test, test_result
            );
        }
        let dest_monkey = if test_result {
            current.dest_true
        } else {
            current.dest_false
        };
        if verbose {
            println!(
                "    Item with worry {} is thrown to monkey {}.",
                item, dest_monkey
            );
        }
        let current = monkeys.get_mut(dest_monkey).context("Invalid dest")?;
        current.items.push_back(item);
    }
    Ok(())
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Result<Input> {
    input.split("\n\n").map(|s| s.parse()).collect()
}

#[aoc(day11, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut input = input.clone();
    for _ in 0..20 {
        for idx in 0..input.len() {
            action(&mut input, idx, false)?;
        }
    }

    let result = input
        .iter()
        .map(|m| m.inspections)
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(2)
        .product();

    Ok(result)
}

#[aoc(day11, part2)]
fn part2(input: &Input) -> Result<Output> {
    let reduction = input.iter().map(|m| m.test).fold(1, |a, b| a.lcm(&b));
    let mut input = input.clone();
    for _ in 0..10000 {
        for idx in 0..input.len() {
            action2(&mut input, idx, reduction, false)?;
        }
    }

    let result = input
        .iter()
        .map(|m| m.inspections)
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(2)
        .product();

    Ok(result)
}

#[aoc(day11, part2, heap)]
fn part2_heap(input: &Input) -> Result<Output> {
    let reduction = input.iter().map(|m| m.test).fold(1, |a, b| a.lcm(&b));
    let mut input = input.clone();
    for _ in 0..10000 {
        for idx in 0..input.len() {
            action2(&mut input, idx, reduction, false)?;
        }
    }

    let result = input
        .iter()
        .map(|m| m.inspections)
        .max_n(2)
        .iter()
        .product();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_test() -> Result<()> {
        let mut input = input_generator(INPUT_STR)?;
        for idx in 0..input.len() {
            action(&mut input, idx, true)?;
        }

        println!("{:?}", input);
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 10605);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 2713310158);
        Ok(())
    }
}

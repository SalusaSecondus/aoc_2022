use std::collections::HashMap;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(HashMap<char, u32>, HashMap<char, u32>)>;
type Output = i32;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = vec![];
    for l in input.lines() {
        let mut c = l.chars();
        let pocket_size = l.len() / 2;
        let mut first = HashMap::new();
        let mut second = HashMap::new();
        for _ in 0..pocket_size {
            *first.entry(c.next().unwrap()).or_default() += 1;
        }
        for _ in 0..pocket_size {
            *second.entry(c.next().unwrap()).or_default() += 1;
        }
        result.push((first, second));
    }

    Ok(result)
}

fn priority(item: char) -> i32 {
    if ('a'..='z').contains(&item) {
        item as i32 - 'a' as i32 + 1
    } else {
        item as i32 - 'A' as i32 + 27
    }
}

fn merge_bags(elf: &(HashMap<char, u32>, HashMap<char, u32>)) -> HashMap<char, u32> {
    let mut result = HashMap::new();
    for e in elf.0.keys() {
        result.insert(*e, 1);
    }
    for e in elf.1.keys() {
        result.insert(*e, 1);
    }
    result
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut result = 0;
    for bag in input {
        for item in bag.0.keys() {
            if bag.1.contains_key(item) {
                println!("{} {}", item, priority(*item));
                result += priority(*item);
            }
        }
    }
    Ok(result)
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> Result<Output> {
    let mut result = 0;
    for group in input.chunks_exact(3) {
        let items1 = merge_bags(&group[0]);
        let items2 = merge_bags(&group[1]);
        let items3 = merge_bags(&group[2]);

        for guess in items1.keys() {
            if items2.contains_key(guess) && items3.contains_key(guess) {
                // println!("{}", guess);
                result += priority(*guess);
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 157);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 70);
        Ok(())
    }
}

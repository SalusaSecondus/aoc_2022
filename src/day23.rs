use std::collections::{HashMap, HashSet};

use anyhow::{Result, Context};
use aoc_runner_derive::{aoc, aoc_generator};
use strum::{EnumIter, IntoEnumIterator};

type Coord = (i32, i32);
type Input = HashSet<Coord>;
type Output = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn checks(&self, loc: &Coord) -> [Coord; 3] {
        match self {
            Direction::North => [
                (loc.0 - 1, loc.1 - 1),
                (loc.0, loc.1 - 1),
                (loc.0 + 1, loc.1 - 1),
            ],
            Direction::South => [
                (loc.0 - 1, loc.1 + 1),
                (loc.0, loc.1 + 1),
                (loc.0 + 1, loc.1 + 1),
            ],
            Direction::West => [
                (loc.0 - 1, loc.1 - 1),
                (loc.0 - 1, loc.1),
                (loc.0 - 1, loc.1 + 1),
            ],
            Direction::East => [
                (loc.0 + 1, loc.1 - 1),
                (loc.0 + 1, loc.1),
                (loc.0 + 1, loc.1 + 1),
            ],
        }
    }

    fn propose(&self, loc: &Coord) -> Coord {
        match self {
            Direction::North => (loc.0, loc.1 - 1),
            Direction::South => (loc.0, loc.1 + 1),
            Direction::East => (loc.0 + 1, loc.1),
            Direction::West => (loc.0 - 1, loc.1),
        }
    }
}
#[aoc_generator(day23)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.insert((x as i32, y as i32));
            }
        }
    }

    Ok(result)
}

fn bounding_box(map: &Input) -> (Coord, Coord) {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    for c in map {
        min.0 = min.0.min(c.0);
        min.1 = min.1.min(c.1);
        max.0 = max.0.max(c.0);
        max.1 = max.1.max(c.1);
    }
    (min, max)
}

#[allow(dead_code)]
fn print_map(map: &Input) {
    let (min, max) = bounding_box(map);
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn adjacent(coord: &Coord) -> [Coord; 8] {
    [(coord.0 - 1, coord.1 - 1), (coord.0, coord.1 - 1), (coord.0 + 1, coord.1 - 1),
    (coord.0 - 1, coord.1), /*                 */ (coord.0 + 1, coord.1),
    (coord.0 - 1, coord.1 + 1), (coord.0, coord.1 + 1), (coord.0 + 1, coord.1 + 1)]
}

fn round<I>(map: &mut Input, directions: &I) -> Result<i32>
where
    I: Iterator<Item = Direction> + Clone,
{
    let mut result = 0;
    let mut proposals: HashMap<(i32, i32), i32> = HashMap::new();
    let mut plans = HashMap::new();

    for elf in map.iter() {
        if !adjacent(elf).iter().any(|elf| map.contains(elf)) {
            continue;
        }
        for d in directions.clone().take(4) {
            let valid = d.checks(elf).iter().all(|elf| !map.contains(elf));
            if valid {
                // println!("{:?} {:?}", elf, d);
                plans.insert(*elf, d.propose(elf));
                *proposals.entry(d.propose(elf)).or_default() += 1;
                break;
            }
        }
    }
    // println!("plans {:?}", plans);

    // println!("proposals {:?}", proposals);

    for (elf, plan) in plans.iter() {
        if proposals.get(plan).context("Missing proposal count")? == &1 {
            map.remove(elf);
            map.insert(*plan);
            result += 1;
        }
    }

    Ok(result)
}

#[aoc(day23, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut map = input.clone();
    let mut directions = Direction::iter().cycle();
    for _round in 1..=10 {
        round(&mut map, &directions)?;
        directions.next();

        // println!("== End of Round {} ==", _round);
        // print_map(&map);
    }
    let (min, max) = bounding_box(&map);
    // print_map(&map);
    let area = (max.0 - min.0 + 1) * (max.1 - min.1 + 1);
    Ok(area - map.len() as i32)
}

#[aoc(day23, part2)]
fn part2(input: &Input) -> Result<Output> {
    let mut map = input.clone();
    let mut directions = Direction::iter().cycle();
    let mut idx = 1;
    while round(&mut map, &directions)? != 0 {
        idx += 1;
        directions.next();
    }
    Ok(idx)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    const INPUT_STR2: &str = ".....
..##.
..#..
.....
..##.
.....";

    #[test]
    fn part1_test() -> Result<()> {
        let mut input = input_generator(INPUT_STR2)?;
        let mut directions = Direction::iter().cycle();
        print_map(&input);
        round(&mut input, &directions)?;
        print_map(&input);
        directions.next();
        round(&mut input, &directions)?;
        print_map(&input);
        directions.next();
        round(&mut input, &directions)?;
        print_map(&input);
        let mut input = input_generator(INPUT_STR)?;
        // todo!();
        assert_eq!(part1(&input)?, 110);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 20);
        Ok(())
    }
}

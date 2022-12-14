use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::{self, complete::i16},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Coord = (i16, i16);
#[derive(Debug, Clone)]
struct Input {
    map: HashMap<Coord, Obstacle>,
    min: Coord,
    max: Coord,
    path: Vec<Coord>,
}
type Output = i16;

#[derive(Clone, Copy, Debug)]
enum Obstacle {
    Rock,
    Sand,
}

fn parse_coord(s: &str) -> IResult<&str, Coord> {
    separated_pair(i16, character::complete::char(','), i16)(s)
}

fn parse_rock(s: &str) -> Result<Vec<Coord>> {
    match separated_list1(tag(" -> "), parse_coord)(s) {
        Ok(o) => Ok(o.1),
        Err(e) => bail!("Could not parse: {}", e),
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Result<Input> {
    let rocks: Vec<Vec<Coord>> = input
        .lines()
        .map(parse_rock)
        .collect::<Result<Vec<Vec<Coord>>>>()?;

    let mut min = (i16::MAX, i16::MAX);
    let mut max = (i16::MIN, i16::MIN);
    let mut map = HashMap::new();
    for r in rocks {
        let mut i = r.iter();
        let curr = i.next().context("No first point")?;
        min.0 = min.0.min(curr.0);
        min.1 = min.1.min(curr.1);
        max.0 = max.0.max(curr.0);
        max.1 = max.1.max(curr.1);

        let mut curr = *curr;
        map.insert(curr, Obstacle::Rock);
        for next_point in i {
            min.0 = min.0.min(next_point.0);
            min.1 = min.1.min(next_point.1);
            max.0 = max.0.max(next_point.0);
            max.1 = max.1.max(next_point.1);
            while curr != *next_point {
                if curr.0 < next_point.0 {
                    curr.0 += 1;
                }
                if curr.0 > next_point.0 {
                    curr.0 -= 1;
                }
                if curr.1 < next_point.1 {
                    curr.1 += 1;
                }
                if curr.1 > next_point.1 {
                    curr.1 -= 1;
                }
                map.insert(curr, Obstacle::Rock);
            }
        }
    }
    Ok(Input {
        map,
        min,
        max,
        path: vec![(500, 0)],
    })
}

#[allow(dead_code)]
fn print_map(input: &Input) {
    for y in i16::min(input.min.1, 0)..input.max.1 + 2 {
        for x in input.min.0 - 10..=input.max.0 + 10 {
            let sym = if let Some(obs) = input.map.get(&(x, y)) {
                match obs {
                    Obstacle::Rock => '#',
                    Obstacle::Sand => 'o',
                }
            } else {
                '.'
            };
            print!("{}", sym);
        }
        println!()
    }
    println!()
}

fn drop_sand(input: &mut Input) -> bool {
    let mut curr_pos = (500, 0);
    while curr_pos.1 <= input.max.1 {
        if !input.map.contains_key(&(curr_pos.0, curr_pos.1 + 1)) {
            curr_pos.1 += 1;
        } else if !input.map.contains_key(&(curr_pos.0 - 1, curr_pos.1 + 1)) {
            curr_pos.0 -= 1;
            curr_pos.1 += 1;
        } else if !input.map.contains_key(&(curr_pos.0 + 1, curr_pos.1 + 1)) {
            curr_pos.0 += 1;
            curr_pos.1 += 1;
        } else {
            input.map.insert(curr_pos, Obstacle::Sand);
            return false;
        }
    }
    true
}

fn drop_sand2(input: &mut Input) {
    let mut curr_pos = (500, 0);
    loop {
        // println!("Current position: {:?}", curr_pos);
        if curr_pos.1 == input.max.1 + 1 {
            input.map.insert(curr_pos, Obstacle::Sand);
            return;
        } else if !input.map.contains_key(&(curr_pos.0, curr_pos.1 + 1)) {
            curr_pos.1 += 1;
        } else if !input.map.contains_key(&(curr_pos.0 - 1, curr_pos.1 + 1)) {
            curr_pos.0 -= 1;
            curr_pos.1 += 1;
        } else if !input.map.contains_key(&(curr_pos.0 + 1, curr_pos.1 + 1)) {
            curr_pos.0 += 1;
            curr_pos.1 += 1;
        } else {
            input.map.insert(curr_pos, Obstacle::Sand);
            return;
        }
    }
}

fn drop_sand3(input: &mut Input) -> bool {
    let path = &mut input.path;
    let mut curr_pos = path.pop().unwrap();
    // head up
    while input.map.contains_key(&curr_pos) {
        curr_pos = path.pop().unwrap();
    }

    // head down
    loop {
        path.push(curr_pos);

        // println!("Current position: {:?}", curr_pos);
        if curr_pos.1 == input.max.1 + 1 {
            input.map.insert(curr_pos, Obstacle::Sand);
            return true;
        } else if !input.map.contains_key(&(curr_pos.0, curr_pos.1 + 1)) {
            curr_pos.1 += 1;
        } else if !input.map.contains_key(&(curr_pos.0 - 1, curr_pos.1 + 1)) {
            curr_pos.0 -= 1;
            curr_pos.1 += 1;
        } else if !input.map.contains_key(&(curr_pos.0 + 1, curr_pos.1 + 1)) {
            curr_pos.0 += 1;
            curr_pos.1 += 1;
        } else {
            input.map.insert(curr_pos, Obstacle::Sand);
            return false;
        }
    }
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut input = input.clone();
    let mut steps = 0;
    while !drop_sand(&mut input) {
        steps += 1;
    }
    Ok(steps)
}

#[aoc(day14, part1, fast)]
fn part1_fast(input: &Input) -> Result<Output> {
    let mut input = input.clone();
    let mut steps = 0;
    while !drop_sand3(&mut input) {
        steps += 1;
    }

    Ok(steps)
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> Result<Output> {
    let mut input = input.clone();
    let mut steps = 0;
    while !input.map.contains_key(&(500, 0)) {
        drop_sand2(&mut input);
        steps += 1;
    }

    Ok(steps)
}

#[aoc(day14, part2, fast)]
fn part2_fast(input: &Input) -> Result<Output> {
    let mut input = input.clone();
    let mut steps = 0;
    while !input.map.contains_key(&(500, 0)) {
        drop_sand3(&mut input);
        steps += 1;
    }
    // print_map(&input);

    Ok(steps)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        // print_map(&input);
        // while !drop_sand(&mut input) {
        //     print_map(&input);
        // }

        assert_eq!(part1(&input)?, 24);
        assert_eq!(part1_fast(&input)?, 24);
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
        assert_eq!(part2(&input)?, 93);
        assert_eq!(part2_fast(&input)?, 93);
        Ok(())
    }
}

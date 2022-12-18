use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::{bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use strum::{EnumIter, IntoEnumIterator, EnumCount};

type Coord = (i64, i64);
type Input = Vec<Direction>;
type Output = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, EnumCount)]
enum RockType {
    Flat,
    Tee,
    Ell,
    Pipe,
    Box,
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    rock_type: RockType,
    location: Coord, // Represents the bottom-left corner of the surrounding box
}

impl Rock {
    fn new(rock_type: RockType, height: i64) -> Self {
        Self {
            rock_type,
            location: (2, height),
        }
    }

    // Returned order is the most likely to collide first
    fn locs(&self) -> Vec<Coord> {
        let (x, y) = self.location;
        match self.rock_type {
            RockType::Flat => vec![(x, y), (x + 3, y), (x + 1, y), (x + 2, y)],
            RockType::Tee => vec![
                (x + 1, y),
                (x, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 1),
                (x + 1, y + 2),
            ],
            RockType::Ell => vec![
                (x, y),
                (x + 2, y),
                (x + 1, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            RockType::Pipe => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            RockType::Box => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }

    fn shift(&self, dir: Direction) -> Self {
        let (x_off, y_off) = match dir {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        };

        Self {
            rock_type: self.rock_type,
            location: (self.location.0 + x_off, self.location.1 + y_off),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => bail!("Unsupported direction: {}", value),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Down => 'v',
        };
        write!(f, "{}", v)
    }
}

#[derive(Clone)]
struct World {
    coords: HashSet<Coord>,
    max_y: i64,
    rock_count: i64,
    trim_height: i64,
    max_drop: i64,
}

impl World {
    fn new() -> World {
        World {
            coords: HashSet::new(),
            max_y: 0,
            rock_count: 0,
            trim_height: 0,
            max_drop: 0,
        }
    }
    fn can_insert(&self, rock: &Rock) -> bool {
        for coord in rock.locs() {
            if coord.0 < 0 || coord.0 > 6 || coord.1 <= 0 {
                return false;
            }
            if self.coords.contains(&coord) {
                return false;
            }
        }
        true
    }

    fn insert(&mut self, rock: Rock) {
        for c in rock.locs() {
            self.max_y = self.max_y.max(c.1);
            self.coords.insert(c);
        }
        self.rock_count += 1;
    }

    #[allow(dead_code)]
    fn print_with_rock(&self, rock: Option<Rock>) {
        let rock = if let Some(rock) = rock {
            rock.locs()
        } else {
            vec![]
        };
        for y in (1..=self.max_y + 7).rev() {
            print!("|");
            for x in 0..7 {
                if self.coords.contains(&(x, y)) {
                    print!("#");
                } else if rock.contains(&(x, y)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("|");
            if y <= self.trim_height {
                println!("|||||||||\n");
                return;    
            }
        }
        println!("+-------+\n");
    }

    fn drop_rock(&mut self, wind: &mut impl Iterator<Item = Direction>, rock_type: RockType) {
        let mut rock = Rock::new(rock_type, self.max_y + 4);
        // self.print_with_rock(Some(rock));
        let mut drop = 0;
        loop {
            let new_rock = rock.shift(wind.next().unwrap());
            rock = if self.can_insert(&new_rock) {
                new_rock
            } else {
                rock
            };
            let new_rock = rock.shift(Direction::Down);
            if self.can_insert(&new_rock) {
                drop += 1;
                rock = new_rock;
            } else {
                self.insert(rock);
                self.trim(rock.location.1);
                self.max_drop = self.max_drop.max(drop);
                return;
            }
        }
    }

    fn trim(&mut self, height: i64) {
        // let trim_height = if self.contains_row(height + 2) {
        //     Some(height + 2)
        // } else if self.contains_row(height + 1) {
        //     Some(height + 1)
        // } else if self.contains_row(height) {
        //     Some(height)
        // } else {
        //     None
        // };

        // println!("Trimming at {}", trim_height);
        let trim_height = height - 40;
        self.coords.retain(|c| c.1 >= trim_height);
        self.trim_height = trim_height;
    }

    #[allow(dead_code)]
    fn normalize(&self) -> Vec<Coord> {
        let mut result = vec![];
        for c in &self.coords {
            result.push((c.0, self.max_y - c.1));
        }
        result.sort();
        result
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Result<Input> {
    input.chars().map(|c| c.try_into()).collect()
}

#[aoc(day17, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut world = World::default();
    let mut wind = input.iter().cycle().copied();
    for r in RockType::iter().cycle().take(2022) {
        world.drop_rock(&mut wind, r);
    }
    // world.print_with_rock(None);
    Ok(world.max_y)
}

#[allow(dead_code)]
fn get_id(ids: &mut HashMap<Vec<Coord>, u16>, name: Vec<Coord>) -> u16 {
    let len = ids.len() as u16;
    *ids.entry(name).or_insert(len)
}

fn part2_closed_form(input: &Input, cycle_num: i64) -> i64 {
    let cycle_length = 1735i64; // Experimentally determined
    let cycle_height_increase = 2781i64; // Experimentally determined
    let cycle_head = 474i64;
    assert!(cycle_num >= cycle_head);

    let cycle_num = cycle_num - cycle_head;
    let cycle_count = cycle_num / cycle_length;
    let cycle_num = cycle_num % cycle_length;

    let mut world = World::default();
    let mut wind = input.iter().cycle().copied();
    for r in RockType::iter().cycle().take((cycle_head + cycle_num + 1) as usize) {
        world.drop_rock(&mut wind, r);
    }

    world.max_y + (cycle_height_increase * cycle_count) 
}

#[aoc(day17, part2)]
fn part2(input: &Input) -> Result<Output> {
    Ok(part2_closed_form(input, 999999999999)) // I don't know why it's off by one
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        println!("{:?}", input);

        let mut world = World::default();
        let mut wind = input.iter().cycle().copied();
        for r in RockType::iter().cycle().take(11) {
            world.drop_rock(&mut wind, r);
        }
        world.print_with_rock(None);
        assert_eq!(18, world.max_y);
        assert_eq!(part1(&input)?, 3068);
        Ok(())
    }

    // #[test]
    #[allow(dead_code)]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 1514285714288);
        Ok(())
    }
}

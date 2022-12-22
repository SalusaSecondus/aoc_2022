use std::{collections::HashMap, fmt::Display, ops::RangeInclusive};

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use num_integer::Roots;

type Coord = (i32, i32);
struct Input {
    map: Map,
    start: Coord,
    instructions: Vec<String>,
}

struct Map {
    tiles: HashMap<Coord, Tile>,
    row_limits: HashMap<i32, RangeInclusive<i32>>,
    col_limits: HashMap<i32, RangeInclusive<i32>>,
    cube_edge: i32,
}

type Output = i32;

struct CubeMap {
    sides: Vec<HashMap<Coord, Tile>>,
    cube_edge: i32,
}

impl TryFrom<&Map> for CubeMap {
    type Error = anyhow::Error;

    fn try_from(map: &Map) -> Result<Self, Self::Error> {
        let cube_edge = map.cube_edge;
        let real = cube_edge == 50;
        let mut sides = vec![];
        sides.resize_with(6 + 1, HashMap::new);
        for (coord, tile) in &map.tiles {
            let side = get_side(*coord, map);
            let coord = (coord.0 % cube_edge, coord.1 % cube_edge);
            let coord = if real {
                match side {
                    1 => coord,
                    2 => (cube_edge - coord.1 - 1, coord.0),
                    3 => (cube_edge - coord.1 - 1, coord.0),
                    4 => (cube_edge - coord.1 - 1, coord.0),
                    5 => coord,
                    6 => coord,
                    _ => bail!("Unsupported side"),
                }
            } else {
                match side {
                    1 => coord,
                    2 => coord,
                    3 => coord,
                    4 => (coord.1, cube_edge - coord.0 - 1),
                    5 => coord,
                    6 => coord, // Top to left
                    _ => bail!("Unsupported side"),
                }
            };
            sides[side].insert(coord, *tile);
        }
        Ok(Self { sides, cube_edge })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Walker {
    loc: Coord,
    direction: Direction,
    side: usize,
}

impl Walker {
    fn step(&self) -> Coord {
        match self.direction {
            Direction::Right => (self.loc.0 + 1, self.loc.1),
            Direction::Down => (self.loc.0, self.loc.1 + 1),
            Direction::Left => (self.loc.0 - 1, self.loc.1),
            Direction::Up => (self.loc.0, self.loc.1 - 1),
        }
    }

    fn score(&self) -> i32 {
        let dir_value = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        1000 * (self.loc.1 + 1) + 4 * (self.loc.0 + 1) + dir_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
enum Tile {
    Empty,
    Wall,
    #[default]
    OffGrid,
}

impl Default for &Tile {
    fn default() -> Self {
        &Tile::OffGrid
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::OffGrid => write!(f, " "),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            ' ' => Tile::OffGrid,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => bail!("Invalid tile: {}", value),
        })
    }
}

impl Direction {
    fn rotate(&self, sym: &str) -> Result<Direction> {
        let result = match sym {
            "R" => match self {
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            },
            "L" => match self {
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
            },
            _ => bail!("Invalid rotation: {}", sym),
        };

        Ok(result)
    }
}

#[aoc_generator(day22)]
fn input_generator(input: &str) -> Result<Input> {
    let (unparsed_map, unparsed_instructions) = input.split_once("\n\n").context("Bad format")?;
    // println!("Instructions {}", unparsed_instructions);
    let mut map = HashMap::new();
    let mut row_limits = HashMap::new();
    let mut col_limits = HashMap::new();
    let mut start = None;
    // let mut width = None;
    for (y, line) in unparsed_map.lines().enumerate() {
        let y = y as i32;
        for (x, tile_char) in line.chars().enumerate() {
            let x = x as i32;
            let tile: Tile = tile_char.try_into()?;
            if tile == Tile::Empty && start.is_none() {
                start = Some((x, y));
            }
            if tile != Tile::OffGrid {
                let limit = row_limits.entry(y).or_insert_with(|| (x..=x));
                *limit = *limit.start().min(&x)..=*limit.end().max(&x);
                let limit = col_limits.entry(x).or_insert_with(|| (y..=y));
                *limit = *limit.start().min(&y)..=*limit.end().max(&y);
                map.insert((x, y), tile);
            }
        }
    }

    let cube_edge = (map.len() / 6).sqrt() as i32;
    let map = Map {
        tiles: map,
        row_limits,
        col_limits,
        cube_edge,
    };

    // todo!()
    let mut instructions = vec![];
    let mut curr_instruction = String::new();
    for c in unparsed_instructions.chars() {
        if c.is_alphabetic() {
            if !curr_instruction.is_empty() {
                instructions.push(curr_instruction.clone());
                curr_instruction.clear();
                instructions.push(c.to_string());
            }
        } else {
            curr_instruction += &c.to_string();
        }
    }

    if !curr_instruction.is_empty() {
        instructions.push(curr_instruction);
    }
    Ok(Input {
        map,
        start: start.context("No start found")?,
        instructions,
    })
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    let y_range = map.col_limits.values().fold(0..=0, |a, b| {
        *a.start().min(b.start())..=*a.end().max(b.end())
    });
    for y in y_range {
        for x in 0..=*map.row_limits.get(&y).unwrap().end() {
            print!("{}", map.tiles.get(&(x, y)).unwrap_or(&Tile::OffGrid));
        }

        println!();
    }
}

fn handle_instruction(me: &mut Walker, input: &Input, instruction: &str) -> Result<()> {
    let map = &input.map;
    let tiles = &map.tiles;
    if let Ok(new_direction) = me.direction.rotate(instruction) {
        me.direction = new_direction;
        return Ok(());
    } else {
        let steps: i32 = instruction.parse()?;
        for _ in 0..steps {
            let mut new_me = me.step();
            let tile = *tiles.get(&new_me).unwrap_or_default();
            if tile == Tile::Wall {
                return Ok(());
            } else if tile == Tile::Empty {
                me.loc = new_me;
            } else if tile == Tile::OffGrid {
                // println!("Wrapping: {:?}", new_me);
                let row_limit = map.row_limits.get(&new_me.1).unwrap_or(&(0..=0));
                let col_limit = map.col_limits.get(&new_me.0).unwrap_or(&(0..=0));
                new_me = match me.direction {
                    Direction::Right => (*row_limit.start(), new_me.1),
                    Direction::Down => (new_me.0, *col_limit.start()),
                    Direction::Left => (*row_limit.end(), new_me.1),
                    Direction::Up => (new_me.0, *col_limit.end()),
                };
                if tiles.get(&new_me).unwrap_or_default() == &Tile::Empty {
                    me.loc = new_me;
                } else {
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

fn get_side(loc: Coord, map: &Map) -> usize {
    let row = loc.1 / map.cube_edge;
    let col = loc.0 / map.cube_edge;
    if map.cube_edge == 4 {
        // Test case
        match (row, col) {
            (0, 2) => 1,
            (1, 0) => 2,
            (1, 1) => 3,
            (1, 2) => 5,
            (2, 2) => 6,
            (2, 3) => 4,
            _ => panic!("Unsupported location: {:?}", (row, col)),
        }
    } else if map.cube_edge == 50 {
        // Real
        match (row, col) {
            (0, 1) => 1,
            (3, 0) => 2,
            (2, 0) => 3,
            (0, 2) => 4,
            (1, 1) => 5,
            (2, 1) => 6,
            _ => panic!("Unsupported location: {:?}", (row, col)),
        }
    } else {
        panic!("Unsupported cube size {}", map.cube_edge);
    }
}

fn handle_instruction_cube(
    me: &mut Walker,
    map: &CubeMap,
    instruction: &str,
    flat: &Map,
) -> Result<()> {
    let sides = &map.sides;
    let cube_edge = map.cube_edge;
    if let Ok(new_direction) = me.direction.rotate(instruction) {
        me.direction = new_direction;
        return Ok(());
    } else {
        let steps: i32 = instruction.parse()?;
        for _ in 0..steps {
            // println!("{:?} -> {:?}", me, flatten_walker(&me, map)?);
            assert_eq!(
                sides[me.side].get(&me.loc),
                flat.tiles.get(&flatten_walker(me, map)?.loc)
            );
            let new_me = me.step();
            let mut new_me = Walker { loc: new_me, ..*me };

            let tile = *sides[new_me.side].get(&new_me.loc).unwrap_or_default();
            if tile == Tile::Wall {
                return Ok(());
            } else if tile == Tile::Empty {
                *me = new_me;
            } else if tile == Tile::OffGrid {
                // println!("Wrapping: {:?}", new_me);

                if me.side == 1 {
                    match me.direction {
                        Direction::Right => {
                            new_me.side = 4;
                            new_me.direction = Direction::Down;
                            new_me.loc = (cube_edge - me.loc.1 - 1, 0);
                        }
                        Direction::Down => {
                            new_me.side = 5;
                            new_me.direction = Direction::Down;
                            new_me.loc = (me.loc.0, 0);
                        }
                        Direction::Left => {
                            new_me.side = 3;
                            new_me.direction = Direction::Down;
                            new_me.loc = (me.loc.1, 0);
                        }
                        Direction::Up => {
                            new_me.side = 2;
                            new_me.direction = Direction::Down;
                            new_me.loc = (cube_edge - me.loc.0 - 1, 0);
                        }
                    }
                } else if me.side == 2 {
                    match me.direction {
                        Direction::Right => {
                            new_me.side = 3;
                            new_me.direction = Direction::Right;
                            new_me.loc = (0, me.loc.1);
                        }
                        Direction::Down => {
                            new_me.side = 6;
                            new_me.direction = Direction::Up;
                            new_me.loc = (cube_edge - me.loc.0 - 1, cube_edge - 1);
                        }
                        Direction::Left => {
                            new_me.side = 4;
                            new_me.direction = Direction::Left;
                            new_me.loc = (cube_edge - 1, me.loc.1);
                        }
                        Direction::Up => {
                            new_me.side = 1;
                            new_me.direction = Direction::Down;
                            new_me.loc = (cube_edge - me.loc.0 - 1, 0);
                        }
                    }
                } else if me.side == 3 {
                    match me.direction {
                        Direction::Right => {
                            new_me.side = 5;
                            new_me.direction = Direction::Right;
                            new_me.loc = (0, me.loc.1);
                        }
                        Direction::Down => {
                            new_me.side = 6;
                            new_me.direction = Direction::Right;
                            new_me.loc = (0, cube_edge - me.loc.0 - 1);
                        }
                        Direction::Left => {
                            new_me.side = 2;
                            new_me.direction = Direction::Left;
                            new_me.loc = (cube_edge - 1, me.loc.1);
                        }
                        Direction::Up => {
                            new_me.side = 1;
                            new_me.direction = Direction::Right;
                            new_me.loc = (0, me.loc.0);
                        }
                    }
                } else if me.side == 4 {
                    match me.direction {
                        Direction::Right => {
                            new_me.side = 2;
                            new_me.direction = Direction::Right;
                            new_me.loc = (0, me.loc.1);
                        }
                        Direction::Down => {
                            new_me.side = 6;
                            new_me.direction = Direction::Left;
                            new_me.loc = (cube_edge - 1, me.loc.0);
                        }
                        Direction::Left => {
                            new_me.side = 5;
                            new_me.direction = Direction::Left;
                            new_me.loc = (cube_edge - 1, me.loc.1);
                        }
                        Direction::Up => {
                            new_me.side = 1;
                            new_me.direction = Direction::Left;
                            new_me.loc = (cube_edge - 1, cube_edge - me.loc.0 - 1);
                        }
                    }
                } else if me.side == 5 {
                    match me.direction {
                        Direction::Right => {
                            new_me.side = 4;
                            new_me.direction = Direction::Right;
                            new_me.loc = (0, me.loc.1);
                        }
                        Direction::Down => {
                            new_me.side = 6;
                            new_me.direction = Direction::Down;
                            new_me.loc = (me.loc.0, 0);
                        }
                        Direction::Left => {
                            new_me.side = 3;
                            new_me.direction = Direction::Left;
                            new_me.loc = (cube_edge - 1, me.loc.1);
                        }
                        Direction::Up => {
                            new_me.side = 1;
                            new_me.direction = Direction::Up;
                            new_me.loc = (me.loc.0, cube_edge - 1);
                        }
                    }
                } else if me.side == 6 {
                    match me.direction {
                        Direction::Right => {
                            new_me.side = 4;
                            new_me.direction = Direction::Up;
                            new_me.loc = (me.loc.1, cube_edge - 1);
                        }
                        Direction::Down => {
                            new_me.side = 2; // TODO
                            new_me.direction = Direction::Up;
                            new_me.loc = (cube_edge - me.loc.0 - 1, cube_edge - 1);
                        }
                        Direction::Left => {
                            new_me.side = 3;
                            new_me.direction = Direction::Up;
                            new_me.loc = (cube_edge - me.loc.1 - 1, cube_edge - 1);
                        }
                        Direction::Up => {
                            new_me.side = 5;
                            new_me.direction = Direction::Up;
                            new_me.loc = (me.loc.0, cube_edge - 1);
                        }
                    }
                } else {
                    bail!("Unknown side: {}", me.side);
                }
                // println!("\tWrapped: {:?}", flatten_walker(&new_me, map)?);
                // println!(
                //     "\tCube tile: {}\tFlat: {}",
                //     sides[new_me.side].get(&new_me.loc).unwrap_or_default(),
                //     flat.tiles.get(&flatten_walker(&new_me, map)?.loc).unwrap_or_default()
                // );
                if sides[new_me.side].get(&new_me.loc).unwrap_or_default() == &Tile::Empty {
                    *me = new_me;
                } else {
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

#[aoc(day22, part1)]
fn part1(input: &Input) -> Result<Output> {
    // let map = &input.map;
    let mut me = Walker {
        loc: input.start,
        direction: Direction::Right,
        side: 0,
    };

    for instruction in &input.instructions {
        handle_instruction(&mut me, input, instruction)?;
        // println!("{}: {:?}", instruction, me);
    }
    Ok(me.score())
}

#[aoc(day22, part2)]
fn part2(input: &Input) -> Result<Output> {
    let mut me = Walker {
        loc: (0, 0),
        direction: Direction::Right,
        side: 1,
    };

    let cube_map: CubeMap = (&input.map).try_into()?;
    // println!("{:?}", cube_map.sides[3]);

    for instruction in &input.instructions {
        handle_instruction_cube(&mut me, &cube_map, instruction, &input.map)?;
        // println!(
        //     "{}: {:?}\t{:?}",
        //     instruction,
        //     me,
        //     flatten_walker(&me, &cube_map)?
        // );
    }
    let me = flatten_walker(&me, &cube_map)?;
    // println!("{:?}", me);
    Ok(me.score())
}

fn flatten_walker(walker: &Walker, map: &CubeMap) -> Result<Walker> {
    let cube_edge = map.cube_edge;
    let real = cube_edge == 50;
    let loc = walker.loc;
    let direction = walker.direction;
    let (loc, direction) = if real {
        match walker.side {
            1 => ((loc.0 + cube_edge, loc.1), direction),
            2 => ((loc.1, 4 * cube_edge - loc.0 - 1), direction.rotate("L")?),
            3 => ((loc.1, 3 * cube_edge - loc.0 - 1), direction.rotate("L")?),
            4 => (
                (2 * cube_edge + loc.1, cube_edge - loc.0 - 1),
                direction.rotate("L")?,
            ),
            5 => ((loc.0 + cube_edge, loc.1 + cube_edge), direction),
            6 => ((loc.0 + cube_edge, loc.1 + 2 * cube_edge), direction),
            _ => bail!("Unsupported side: {}", walker.side),
        }
    } else {
        match walker.side {
            1 => ((loc.0 + 2 * cube_edge, loc.1), direction),
            2 => ((loc.0, loc.1 + cube_edge), direction),
            3 => ((loc.0 + cube_edge, loc.1 + cube_edge), direction),
            4 => (
                (4 * cube_edge - loc.1 - 1, 2 * cube_edge + loc.0),
                walker.direction.rotate("R")?,
            ),
            5 => ((loc.0 + 2 * cube_edge, loc.1 + cube_edge), direction),
            6 => ((loc.0 + 2 * cube_edge, loc.1 + 2 * cube_edge), direction),
            _ => bail!("Unsupported side: {}", walker.side),
        }
    };
    Ok(Walker {
        loc,
        direction,
        side: 0,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "        ...#     
        .#..     
        #...     
        ....     
...#.......#     
........#...     
..#....#....     
..........#.     
        ...#....
        .....#.. 
        .#...... 
        ......#. 

10R5L5R10L4R5L5";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        print_map(&input.map);
        println!("{:?}", input.start);
        assert_eq!(part1(&input)?, 6032);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        // let cube_map: CubeMap = (&input.map).try_into()?;
        assert_eq!(part2(&input)?, 5031);
        Ok(())
    }
}

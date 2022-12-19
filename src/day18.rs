use std::collections::HashSet;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type Coord = (i16, i16, i16);
struct Input {
    map: HashSet<Coord>,
    max: Coord,
    min: Coord,
}
type Output = i32;

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = HashSet::new();
    let mut max_a = i16::MIN;
    let mut max_b = i16::MIN;
    let mut max_c = i16::MIN;
    let mut min_a = i16::MAX;
    let mut min_b = i16::MAX;
    let mut min_c = i16::MAX;
    for l in input.lines() {
        let mut parts = l.split(',');
        let a = parts.next().context("No part")?.parse()?;
        let b = parts.next().context("No part")?.parse()?;
        let c = parts.next().context("No part")?.parse()?;
        max_a = max_a.max(a);
        min_a = min_a.min(a);
        max_b = max_b.max(b);
        min_b = min_b.min(b);
        max_c = max_c.max(c);
        min_c = min_c.min(c);
        result.insert((a, b, c));
    }
    Ok(Input {
        map: result,
        max: (max_a, max_b, max_c),
        min: (min_a, min_b, min_c),
    })
}

#[aoc(day18, part1)]
fn part1(input: &Input) -> Result<Output> {
    println!("Min: {:?}", input.min);
    println!("Max: {:?}", input.max);
    let map = &input.map;
    let mut border = map.len() * 6;
    for c in map {
        if map.contains(&(c.0 - 1, c.1, c.2)) {
            border -= 1;
        }
        if map.contains(&(c.0 + 1, c.1, c.2)) {
            border -= 1;
        }
        if map.contains(&(c.0, c.1 - 1, c.2)) {
            border -= 1;
        }
        if map.contains(&(c.0, c.1 + 1, c.2)) {
            border -= 1;
        }
        if map.contains(&(c.0, c.1, c.2 - 1)) {
            border -= 1;
        }
        if map.contains(&(c.0, c.1, c.2 + 1)) {
            border -= 1;
        }
    }

    Ok(border as i32)
}

#[aoc(day18, part2)]
fn part2(input: &Input) -> Result<Output> {
    let (min_x, min_y, min_z) = (input.min.0 - 1, input.min.1 - 1, input.min.2 - 1);
    let (max_x, max_y, max_z) = (input.max.0 + 1, input.max.1 + 1, input.max.2 + 1);
    let map = &input.map;

    let mut visited = HashSet::new();

    let mut queue = vec![(min_x, min_y, min_z)];
    let mut border = 0;

    while let Some(c) = queue.pop() {
        if visited.contains(&c) {
            continue;
        }
        visited.insert(c);
        if c.0 > min_x {
            if map.contains(&(c.0 - 1, c.1, c.2)) {
                border += 1;
            } else if !visited.contains(&(c.0 - 1, c.1, c.2)) {
                queue.push((c.0 - 1, c.1, c.2));
            }
        }
        if c.1 > min_y {
            if map.contains(&(c.0, c.1 - 1, c.2)) {
                border += 1;
            } else if !visited.contains(&(c.0, c.1 - 1, c.2)) {
                queue.push((c.0, c.1 - 1, c.2));
            }
        }
        if c.2 > min_z {
            if map.contains(&(c.0, c.1, c.2 - 1)) {
                border += 1;
            } else if !visited.contains(&(c.0, c.1, c.2 - 1)) {
                queue.push((c.0, c.1, c.2 - 1));
            }
        }
        if c.0 < max_x {
            if map.contains(&(c.0 + 1, c.1, c.2)) {
                border += 1;
            } else if !visited.contains(&(c.0 + 1, c.1, c.2)) {
                queue.push((c.0 + 1, c.1, c.2));
            }
        }
        if c.1 < max_y {
            if map.contains(&(c.0, c.1 + 1, c.2)) {
                border += 1;
            } else if !visited.contains(&(c.0, c.1 + 1, c.2)) {
                queue.push((c.0, c.1 + 1, c.2));
            }
        }
        if c.2 < max_z {
            if map.contains(&(c.0, c.1, c.2 + 1)) {
                border += 1;
            } else if !visited.contains(&(c.0, c.1, c.2 + 1)) {
                queue.push((c.0, c.1, c.2 + 1));
            }
        }
    }
    Ok(border)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;

        assert_eq!(part1(&input)?, 64);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 58);
        Ok(())
    }
}

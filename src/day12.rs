use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use salusa_aoc::{Graph, MatrixTranspose};

type Coord = (i32, i32);
type Input = (Graph<Coord>, Coord, Coord, Vec<Vec<i8>>);
type Output = u32;

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Result<Input> {
    let elems = input
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();

    let mut start = Coord::default();
    let mut end = Coord::default();

    let mut heights = vec![vec![0i8; elems.len()]; elems[0].len()];
    for (y, row) in elems.iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            let height = match e {
                'S' => {
                    start = (x as i32, y as i32);
                    1
                }
                'E' => {
                    end = (x as i32, y as i32);
                    26
                }
                _ => *e as i8 + 1i8 - b'a' as i8,
            };
            heights[x][y] = height;
        }
    }

    let max_x = heights.len() - 1;
    let max_y = heights[0].len() - 1;
    let mut graph: Graph<Coord> = Graph::new(false);
    for (x, col) in heights.iter().enumerate() {
        for (y, curr_height) in col.iter().enumerate() {
            // let curr_height = heights[x][y];
            if x > 0 && can_move(*curr_height, heights[x - 1][y]) {
                graph.add_edge((x as i32, y as i32), (x as i32 - 1, y as i32));
            }
            if x < max_x && can_move(*curr_height, heights[x + 1][y]) {
                graph.add_edge((x as i32, y as i32), (x as i32 + 1, y as i32));
            }
            if y > 0 && can_move(*curr_height, heights[x][y - 1]) {
                graph.add_edge((x as i32, y as i32), (x as i32, y as i32 - 1));
            }
            if y < max_y && can_move(*curr_height, heights[x][y + 1]) {
                graph.add_edge((x as i32, y as i32), (x as i32, y as i32 + 1));
            }
        }
    }

    Ok((graph, start, end, heights))
}

fn can_move(curr_height: i8, end_height: i8) -> bool {
    curr_height >= end_height || end_height - curr_height == 1
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut dists: HashMap<Coord, u32> = HashMap::new();
    let graph = &input.0;
    let start = input.1;
    let end = input.2;
    dists.insert(start, 0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let my_dist = *dists.get(&node).context("No dist to current node")?;
        if let Some(edges) = graph.edges(&node) {
            for e in edges {
                if !dists.contains_key(e) {
                    dists.insert(*e, my_dist + 1);
                    queue.push_back(*e);
                }
                if *e == end {
                    return Ok(my_dist + 1);
                }
            }
        }
    }

    bail!("No path found");
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> Result<Output> {
    let mut dists: HashMap<Coord, u32> = HashMap::new();
    let graph = input.0.transpose();
    let start = input.2;
    let heights = &input.3;
    dists.insert(start, 0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut best_dist = u32::MAX;

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let my_dist = *dists.get(&node).context("No dist to current node")?;
        if my_dist >= best_dist - 1 {
            continue;
        }
        if let Some(edges) = graph.edges(&node) {
            for e in edges {
                if !dists.contains_key(e) {
                    dists.insert(*e, my_dist + 1);
                    queue.push_back(*e);
                    if heights[e.0 as usize][e.1 as usize] == 1 && my_dist + 1 < best_dist {
                        // println!("Found new best {} at {:?}", my_dist + 1, e);
                        best_dist = my_dist + 1;
                    }
                }
            }
        }
    }

    Ok(best_dist)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 31);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 29);
        Ok(())
    }
}

use std::{cmp::Ordering, collections::HashSet, str::FromStr};

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type Coord = (i32, i32);
type SimpleRange = (i32, i32);
type Input = Vec<Sensor>;
type Output = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sensor {
    loc: Coord,
    beacon: Coord,
}

impl Sensor {
    fn dist(&self, other: &Coord) -> i32 {
        dist(&self.loc, other)
    }

    fn max_dist(&self) -> i32 {
        self.dist(&self.beacon)
    }
}

fn cmp_simple_range(a: &SimpleRange, b: &SimpleRange) -> Ordering {
    match a.0.cmp(&b.0) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a.1.cmp(&b.1),
    }
}

fn maybe_merge(a: SimpleRange, b: SimpleRange) -> Vec<SimpleRange> {
    // print!("{:?} + {:?} = ", a, b);
    let (a, b) = match cmp_simple_range(&a, &b) {
        Ordering::Less => (a, b),
        Ordering::Equal => {
            println!("a");
            return vec![a];
        }
        Ordering::Greater => (b, a),
    };
    // a is now in front of b
    if a.1 < b.0 {
        // No overlap
        // println!("Unchanged");
        vec![a, b]
    } else if a.1 >= b.1 {
        // A fully contains b
        // println!("a");
        vec![a]
    } else {
        // println!("{:?}", (a.0, b.1));
        vec![(a.0, b.1)]
    }
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "^Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)"
            )
            .unwrap();
        }
        let m = RE.captures(s).context("Bad pattern")?;
        let x = m.get(1).context("No x")?.as_str();
        let y = m.get(2).context("No y")?.as_str();
        let loc = (x.parse()?, y.parse()?);
        let x = m.get(3).context("No x")?.as_str();
        let y = m.get(4).context("No y")?.as_str();
        let beacon = (x.parse()?, y.parse()?);

        Ok(Self { loc, beacon })
    }
}

fn dist(a: &Coord, b: &Coord) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[allow(dead_code)]
fn print_sensor_chances(sensors: &[Sensor], coord: &Coord) {
    for (idx, s) in sensors.iter().enumerate() {
        let max_dist = s.max_dist();
        let dist = s.dist(coord);
        println!(
            "{}\t{:?}:\t{}, ({}, {})",
            idx,
            s.loc,
            dist <= max_dist,
            dist,
            max_dist
        );
    }
}

fn no_beacons(sensors: &[Sensor], row: i32) -> (Vec<SimpleRange>, Vec<i32>) {
    let mut ranges: Vec<SimpleRange> = vec![];
    let mut in_use: HashSet<Coord> = HashSet::new();

    for (_idx, s) in sensors.iter().enumerate() {
        let max_dist = s.max_dist();
        let row_dist = s.dist(&(s.loc.0, row));
        let diff = max_dist - row_dist;
        if diff >= 0 {
            // println!("{}\t{:?}:\t {}", idx, s.loc, diff);
            ranges.push((s.loc.0 - diff, s.loc.0 + diff));
            if s.loc.1 == row {
                in_use.insert(s.loc);
            }
            if s.beacon.1 == row {
                in_use.insert(s.beacon);
            }
        }
    }
    // println!("Ranges: {:?}", ranges);

    ranges.sort_unstable_by(cmp_simple_range);
    // println!("Ranges: {:?}", ranges);
    let mut merged_ranges: Vec<SimpleRange> = vec![];
    let mut i = ranges.iter();
    merged_ranges.push(i.next().copied().unwrap());
    for r in i {
        let tail = merged_ranges.pop().unwrap();
        merged_ranges.extend(maybe_merge(tail, *r));
    }
    // println!("Merged: {:?}", merged_ranges);
    // println!("In use: {:?}", in_use);

    let in_use: Vec<i32> = in_use.iter().map(|c| c.0).sorted().collect_vec();

    (merged_ranges, in_use)
}

fn count_no_beacon(sensors: &[Sensor], row: i32) -> Result<i32> {
    let (merged, in_use) = no_beacons(sensors, row);
    Ok(merged.iter().map(|r| r.1 - r.0 + 1).sum::<i32>() - in_use.len() as i32)
}

fn find_gap(sensors: &[Sensor], max_x: i32, max_y: i32) -> Result<i64> {
    for y in 0..=max_y {
        let (merged, _) = no_beacons(sensors, y);
        // println!("ranges: {:?}, in use: {:?}, (?, {})", merged, in_use, y);
        let mut x = 0;
        let mut ranges = merged.iter();
        let mut curr_range = ranges.next().context("No ranges?")?;
        while x <= max_x {
            if x >= curr_range.0 && x <= curr_range.1 {
                // x is in the range
                x = curr_range.1 + 1;
            } else {
                curr_range = match ranges.next() {
                    Some(r) => r,
                    None => {
                        // println!("ranges: {:?}, in use: {:?}, ({}, {})", merged, in_use, x, y);
                        // print_sensor_chances(sensors, &(x, y));
                        // break;
                        return Ok(x as i64 * 4000000i64 + y as i64);
                    }
                }
            }
        }
        // todo!("Fix me!")
    }
    bail!("Not found");
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Result<Input> {
    input.lines().map(Sensor::from_str).collect()
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> Result<Output> {
    Ok(count_no_beacon(input, 2000000)? as i64)
}

#[aoc(day15, part2)]
fn part2(input: &Input) -> Result<Output> {
    find_gap(input, 4000000, 4000000)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(count_no_beacon(&input, 10)?, 26);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(find_gap(&input, 20, 20)?, 56000011);
        Ok(())
    }
}

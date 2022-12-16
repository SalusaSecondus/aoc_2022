use std::collections::{HashMap, HashSet};

use anyhow::{bail, ensure, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use salusa_aoc::Graph;

#[derive(Debug, Clone)]
struct Input {
    #[allow(dead_code)]
    map: Graph<u16, u32>,
    flows: HashMap<u16, u32>,
    dists: HashMap<u16, HashMap<u16, u32>>,
}

type Output = u32;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Result<Input> {
    let mut map = Graph::new(false);
    let mut flows = HashMap::new();
    lazy_static! {
        static ref RE: Regex =
            Regex::new("^Valve (\\S+) has flow rate=(\\d+); tunnels? leads? to valves? (.+)$")
                .unwrap();
    }

    for line in input.lines() {
        if let Some(m) = RE.captures(line) {
            let name = m.get(1).context("No name")?.as_str();
            let name = name_to_id(name);
            let flow = m.get(2).context("No flow")?.as_str().parse::<u32>()?;

            for out in m.get(3).context("No tunnels")?.as_str().split(", ") {
                map.add_edge(name, name_to_id(out));
            }
            flows.insert(name, flow);
        } else {
            bail!("Could not parse: {}", line);
        }
    }

    let mut dists = HashMap::new();
    for node in flows.keys() {
        dists.insert(*node, map.distance_map(node));
    }

    Ok(Input { map, flows, dists })
}

const fn name_to_id(name: &str) -> u16 {
    let b = name.as_bytes();
    ((b[0] as u16) << 8) +  b[1] as u16
}

fn set_without(set: &HashSet<u16>, node: &u16) -> Result<HashSet<u16>> {
    let mut set = set.clone();
    ensure!(set.remove(node));

    Ok(set)
}

fn sub1(node: u16, input: &Input, time_left: u32, useful: HashSet<u16>) -> Result<u32> {
    let mut prefix = String::new();
    for _ in time_left..30 {
        prefix += "  ";
    }
    if time_left <= 1 {
        return Ok(0);
    } else if time_left == 2 {
        if useful.contains(&node) {
            return input.flows.get(&node).copied().context("No self flow");
        } else {
            return Ok(0);
        }
    }
    let dists = input.dists.get(&node).context("No dists")?;

    let my_val = input.flows.get(&node).context("No self flow")? * (time_left - 1);
    let mut best = my_val;
    let my_cost = if my_val > 0 { 1 } else { 0 };
    // println!("{}Visiting {}({}): Providing {}", prefix, node, 30 - time_left, my_val);

    for next in &useful {
        let step_length = dists.get(next).context("No step")?;
        if step_length + 1 > time_left {
            continue;
        }
        let attempt = sub1(
            *next,
            input,
            time_left - my_cost - step_length,
            set_without(&useful, next)?,
        )?;
        // println!("{} Result {}:", prefix, attempt);
        best = best.max(attempt + my_val)
    }

    Ok(best)
}

const AA: u16 = name_to_id("AA");

#[aoc(day16, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut useful_valves = HashSet::new();
    for (valve, flow) in &input.flows {
        if flow > &0 {
            useful_valves.insert(*valve);
        }
    }
    sub1(AA, input, 30, useful_valves)
}

fn sub2(
    input: &Input,
    me: &mut HashSet<u16>,
    elephant: &mut HashSet<u16>,
    useful: &mut Vec<u16>,
) -> Result<Output> {
    if let Some(next_node) = useful.pop() {
        me.insert(next_node);
        let left = sub2(input, me, elephant, useful)?;
        me.remove(&next_node);
        elephant.insert(next_node);
        let right = sub2(input, me, elephant, useful)?;
        elephant.remove(&next_node);
        useful.push(next_node);
        Ok(left.max(right))
    } else {
        sub1(AA, input, 26, me.clone())
            .and_then(|m| Ok(m + sub1(AA, input, 26, elephant.clone())?))
    }
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> Result<Output> {
    // We're going to brute-force this!
    // My estimate is that there are about 2^14 possible cases which can (probably) be powered though.
    let mut useful_valves = vec![];
    for (valve, flow) in &input.flows {
        if flow > &0 {
            useful_valves.push(*valve);
        }
    }
    let mut me = HashSet::new();
    let mut elephant = HashSet::new();
    me.insert(useful_valves.pop().context("No valves")?);
    sub2(input, &mut me, &mut elephant, &mut useful_valves)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 1651);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 1707);
        Ok(())
    }
}

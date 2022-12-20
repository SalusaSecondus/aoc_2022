use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use salusa_aoc::{Graph, bitset::BitSet};

#[derive(Debug, Clone)]
struct Input {
    #[allow(dead_code)]
    map: Graph<u8, u32>,
    flows: [u32; 64],
    dists: [[u32; 64]; 64],
    aa: u8,
}

type Output = u32;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Result<Input> {
    let mut map = Graph::new(false);
    let mut flows = [0; 64];
    lazy_static! {
        static ref RE: Regex =
            Regex::new("^Valve (\\S+) has flow rate=(\\d+); tunnels? leads? to valves? (.+)$")
                .unwrap();
    }

    let mut name_to_ids = HashMap::new();
    for line in input.lines() {
        if let Some(m) = RE.captures(line) {
            let name = m.get(1).context("No name")?.as_str();
            let name = get_id(&mut name_to_ids, name);

            let flow = m.get(2).context("No flow")?.as_str().parse::<u32>()?;

            for out in m.get(3).context("No tunnels")?.as_str().split(", ") {
                map.add_edge(name, get_id(&mut name_to_ids, out));
            }
            flows[name as usize] = flow;
        } else {
            bail!("Could not parse: {}", line);
        }
    }

    let mut dists = [[u32::MAX; 64]; 64];
    for node in 0u8..64 {
        dists[node as usize] = dist_to_slice(&map.distance_map(&node));
    }

    Ok(Input {
        map,
        flows,
        dists,
        aa: get_id(&mut name_to_ids, "AA"),
    })
}

fn dist_to_slice(map: &HashMap<u8, u32>) -> [u32; 64] {
    let mut result = [u32::MAX; 64];

    for idx in 0u8..64 {
        result[idx as usize] = *map.get(&idx).unwrap_or(&u32::MAX);
    }
    result
}

fn get_id(ids: &mut HashMap<String, u8>, name: &str) -> u8 {
    let len = ids.len() as u8;
    *ids.entry(name.to_string()).or_insert(len)
}

fn set_without(set: &BitSet<u64>, value: &u8) -> BitSet<u64> {
    let mut set = *set;
    set.remove(value);
    set
}

fn sub1(node: u8, input: &Input, time_left: u32, useful: BitSet<u64>) -> Result<u32> {
    let mut prefix = String::new();
    for _ in time_left..30 {
        prefix += "  ";
    }
    if time_left <= 1 {
        return Ok(0);
    } else if time_left == 2 {
        if useful.contains(&node) {
            return Ok(input.flows[node as usize]);
        } else {
            return Ok(0);
        }
    }
    let dists = input.dists[node as usize];

    let my_val = input.flows[node as usize] * (time_left - 1);
    let mut best = my_val;
    let my_cost = (my_val > 0) as u32;
    // println!("{}Visiting {}({}): Providing {}", prefix, node, 30 - time_left, my_val);

    for next in useful {
        let step_length = dists[next as usize];
        if step_length + 1 > time_left {
            continue;
        }
        let attempt = sub1(
            next,
            input,
            time_left - my_cost - step_length,
            set_without(&useful, &next),
        )?;
        // println!("{} Result {}:", prefix, attempt);
        best = best.max(attempt + my_val)
    }

    Ok(best)
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> Result<Output> {
    let mut useful_valves = BitSet::new();
    for (valve, flow) in input.flows.iter().enumerate() {
        if flow > &0 {
            useful_valves.insert(valve as u8);
        }
    }
    sub1(input.aa, input, 30, useful_valves)
}

fn sub2(input: &Input, me: &mut BitSet<u64>, elephant: &mut BitSet<u64>, useful: &mut Vec<u8>) -> Result<Output> {
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
        sub1(input.aa, input, 26, *me).and_then(|m| Ok(m + sub1(input.aa, input, 26, *elephant)?))
    }
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> Result<Output> {
    // We're going to brute-force this!
    // My estimate is that there are about 2^14 possible cases which can (probably) be powered though.
    let mut useful_valves = vec![];
    for (valve, flow) in input.flows.iter().enumerate() {
        if flow > &0 {
            useful_valves.push(valve as u8);
        }
    }
    let mut me = BitSet::new();
    let mut elephant = BitSet::new();
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

use core::time;
use std::{collections::{HashSet, HashMap}, fmt::Display, str::FromStr};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use strum::{EnumCount, EnumDiscriminants, EnumIter, IntoEnumIterator};

type Input = Vec<Blueprint>;
type Output = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    const fn idx(&self) -> usize {
        match self {
            Resource::Ore => 0,
            Resource::Clay => 1,
            Resource::Obsidian => 2,
            Resource::Geode => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    idx: u16,
    recipes: [[u16; Resource::COUNT]; Resource::COUNT],
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            self.idx,
            self.recipes[Resource::Ore.idx()][Resource::Ore.idx()],
            self.recipes[Resource::Clay.idx()][Resource::Ore.idx()],
            self.recipes[Resource::Obsidian.idx()][Resource::Ore.idx()],
            self.recipes[Resource::Obsidian.idx()][Resource::Clay.idx()],
            self.recipes[Resource::Geode.idx()][Resource::Ore.idx()],
            self.recipes[Resource::Geode.idx()][Resource::Obsidian.idx()]
        )
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("Blueprint (\\d+):\\s+Each ore robot costs (\\d+) ore.\\s+Each clay robot costs (\\d+) ore.\\s+Each obsidian robot costs (\\d+) ore and (\\d+) clay.\\s+Each geode robot costs (\\d+) ore and (\\d+) obsidian.").unwrap();
        }
        let groups = RE.captures(s).context("Bad format")?;
        let idx = groups.get(1).context("No index")?.as_str().parse()?;
        let ore_cost = groups.get(2).context("No ore_cost")?.as_str().parse()?;
        let clay_ore = groups.get(3).context("No clay_ore")?.as_str().parse()?;
        let obs_ore = groups.get(4).context("No obs_ore")?.as_str().parse()?;
        let obs_clay = groups.get(5).context("No obs_clay")?.as_str().parse()?;
        let geode_ore = groups.get(6).context("No geode_ore")?.as_str().parse()?;
        let geode_obs = groups.get(7).context("No geode_obs")?.as_str().parse()?;

        let mut recipes = [[0u16; Resource::COUNT]; Resource::COUNT];
        recipes[Resource::Ore.idx()][Resource::Ore.idx()] = ore_cost;
        recipes[Resource::Clay.idx()][Resource::Ore.idx()] = clay_ore;
        recipes[Resource::Obsidian.idx()][Resource::Ore.idx()] = obs_ore;
        recipes[Resource::Obsidian.idx()][Resource::Clay.idx()] = obs_clay;
        recipes[Resource::Geode.idx()][Resource::Ore.idx()] = geode_ore;
        recipes[Resource::Geode.idx()][Resource::Obsidian.idx()] = geode_obs;

        Ok(Blueprint { idx, recipes })
    }
}

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Result<Input> {
    input.split('\n').map(Blueprint::from_str).collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct World {
    resources: [u16; Resource::COUNT],
    bots: [u16; Resource::COUNT],
    time_left: u16,
}

impl World {
    fn new(time_left: u16) -> Self {
        let resources = [0u16; Resource::COUNT];
        let mut bots = [0u16; Resource::COUNT];
        bots[Resource::Ore.idx()] = 1;
        Self { resources, bots, time_left }
    }

    fn step_time(&self, resource: Option<Resource>, blueprint: &Blueprint) -> Option<World> {
        assert!(self.time_left > 0);
        let mut result = *self;
        if let Some(resource) = resource {
            for (resource_idx, cost) in blueprint.recipes[resource.idx()].iter().enumerate() {
                if self.resources[resource_idx] < *cost {
                    return None;
                }
            }
            // We can afford it
            for (resource_idx, cost) in blueprint.recipes[resource.idx()].iter().enumerate() {
                result.resources[resource_idx] -= cost;
            }

            result.bots[resource.idx()] += 1;

        }

        // Purposefully using bots from old world to avoid newly created one
        for (resource_idx, bot_count) in self.bots.iter().enumerate() {
            result.resources[resource_idx] += *bot_count;
        }

        result.time_left -= 1;
        Some(result)
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "World: Ore {} ({}), Clay: {} ({}), Obsidian {} ({}), Geode {} ({})",
            self.resources[Resource::Ore.idx()],
            self.bots[Resource::Ore.idx()],
            self.resources[Resource::Clay.idx()],
            self.bots[Resource::Clay.idx()],
            self.resources[Resource::Obsidian.idx()],
            self.bots[Resource::Obsidian.idx()],
            self.resources[Resource::Geode.idx()],
            self.bots[Resource::Geode.idx()],
    )
    }
}

fn most_geodes(blueprint: &Blueprint, world: World, saved: &mut HashMap<World, u32>) -> u32 {
    if world.time_left == 0 {
        return world.resources[Resource::Geode.idx()] as u32;
    }
    if world.time_left > 1 {
        if let Some(result) = saved.get(&world) {
            // println!("Found saved with {} minutes left. {}", world.time_left, world);
            return *result;
        }
        // println!("Not saved with {} minutes left. {}", world.time_left, world);    
    }

    // Try all possible next steps and find the max
    let mut best = 0;

    if world.time_left == 1 {
        // Not worth creating a new bot
        return most_geodes(blueprint, world.step_time(None, blueprint).unwrap(), saved);
    } else if let Some(next) = world.step_time(Some(Resource::Geode), blueprint) {
    // If we can create a new Geode bot, that's always the thing to do

        return most_geodes(blueprint, next, saved);
    } 

    for new_bot in Resource::iter() {
        if let Some(next) = world.step_time(Some(new_bot), blueprint) {
            // println!("Build a bot for {:?} with {} minutes left. {:?}", new_bot, time_left, next);

            // if new_bot == Resource::Geode {
            //     println!("Build a bot for {:?} with {} minutes left", new_bot, time_left);

            // }
            let guess = most_geodes(blueprint, next, saved);
            best = best.max(guess);
        }
    }
    // Also do nothing!
    let guess = most_geodes(blueprint, world.step_time(None, blueprint).unwrap(), saved);
    best = best.max(guess);
    // println!("Current best: {}", best);
    saved.insert(world, best);
    best
}

#[aoc(day19, part1)]
#[ignore]
fn part1(input: &Input) -> Result<Output> {
    Ok(input
        .iter()
        .map(|bp| {println!("Looking at blueprint {}", bp.idx); bp})
        .map(|bp| bp.idx as u32 * most_geodes(bp, World::new(24), &mut HashMap::new()))
        .sum())
}

#[aoc(day19, part2)]
#[ignore]
fn part2(input: &Input) -> Result<Output> {
    Ok(input
        .iter()
        .take(3)
        .inspect(|bp| println!("Looking at blueprint {}", bp.idx))
        .map(|bp|most_geodes(bp, World::new(32), &mut HashMap::new()))
        .inspect(|geodes| println!("\tMade {} geodes.", geodes))
        .product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "Blueprint 1:    Each ore robot costs 4 ore.   Each clay robot costs 2 ore.    Each obsidian robot costs 3 ore and 14 clay.    Each geode robot costs 2 ore and 7 obsidian.
  Blueprint 2:    Each ore robot costs 2 ore.    Each clay robot costs 3 ore.    Each obsidian robot costs 3 ore and 8 clay.    Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(most_geodes(&input[0], World::new(24), &mut HashMap::new()), 9);
        println!("Blueprint 1 is good!");
        assert_eq!(most_geodes(&input[1], World::new(24), &mut HashMap::new()), 12);
        println!("Blueprint 2 is good!");
        assert_eq!(part1(&input)?, 33);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?.iter().take(3).copied().collect_vec();
        assert_eq!(most_geodes(&input[0], World::new(32), &mut HashMap::new()), 56);
        println!("Blueprint 1 is good!");
        assert_eq!(most_geodes(&input[1], World::new(32), &mut HashMap::new()), 62);
        println!("Blueprint 2 is good!");
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 56*62);
        Ok(())
    }
}

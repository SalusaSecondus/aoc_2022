use std::str::FromStr;

use anyhow::{bail, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Cmd>;
type Output = i32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
enum Cmd {
    NOOP,
    ADDX(i32),
}

impl FromStr for Cmd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Cmd::NOOP)
        } else {
            let parts = s.split_once(' ').context("Improperly formed command")?;
            match parts.0 {
                "addx" => Ok(Cmd::ADDX(parts.1.parse()?)),
                _ => bail!("Unexpected command: {}", s),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CpuState {
    Ready,
    Executing(u32, Cmd),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cpu {
    cycle: u32,
    instruction_pointer: usize,
    register: i32,
    state: CpuState,
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycle: 1,
            instruction_pointer: 0,
            register: 1,
            state: CpuState::Ready,
        }
    }

    fn step(&mut self, instructions: &[Cmd]) -> Result<(u32, i32)> {
        let result = Ok((self.cycle, self.register));
        self.cycle += 1;
        if let CpuState::Executing(delay, cmd) = self.state {
            if delay > 1 {
                self.state = CpuState::Executing(delay - 1, cmd);
            } else {
                // Right now only ADDX is supported...
                if let Cmd::ADDX(change) = cmd {
                    self.register += change;
                } else {
                    todo!("Unsupported command: {:?}", cmd);
                }
                self.state = CpuState::Ready;
                self.instruction_pointer += 1;
            }
        } else {
            // Ready to accept the next command
            let cmd = instructions
                .get(self.instruction_pointer)
                .context("No more instructions")?;
            match cmd {
                Cmd::NOOP => self.instruction_pointer += 1,
                Cmd::ADDX(_) => self.state = CpuState::Executing(1, *cmd),
            };
        }

        result
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Result<Input> {
    let result = input.lines().map(|l| l.trim().parse()).collect();
    result
}

#[aoc(day10, part1)]
#[allow(clippy::comparison_chain)]
fn part1(input: &Input) -> Result<Output> {
    let mut cpu = Cpu::new();
    let mut result = 0;
    while cpu.cycle <= 220 {
        let (cycle, register) = cpu.step(input)?;
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                result += register * cycle as i32;
                // println!("total: {}, {}, {}", result, cycle, register)
            }
            _ => (),
        };
    }
    // result += cpu.register * 220;

    Ok(result)
}

#[aoc(day10, part2)]
#[allow(clippy::comparison_chain)]
fn part2(input: &Input) -> Result<Output> {
    let mut cpu = Cpu::new();
    while cpu.cycle <= 240 {
        let (cycle, register) = cpu.step(input)?;
        let current_position = ((cycle - 1) % 40) as i32;

        let visible = (register - current_position).abs() < 2; // Do we care about registers off the range?
        let sym = if visible { '#' } else { '.' };
        print!("{}", sym);
        if current_position == 39 {
            println!();
        }
    }
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator("noop\naddx 3\naddx -5")?;
        let mut cpu = Cpu::new();
        while let Ok(state) = cpu.step(&input) {
            println!("{:?}", state);
        }
        println!("{:?}", cpu);
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 13140);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 0);
        Ok(())
    }
}

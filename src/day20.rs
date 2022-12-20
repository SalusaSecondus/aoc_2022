use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use anyhow::{ensure, Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
struct ProblemInput {
    nodes: Vec<Rc<RefCell<Node>>>,
    #[allow(dead_code)]
    list: Weak<RefCell<Node>>,
    zero_point: Weak<RefCell<Node>>,
}

const DECRYPTION_KEY: i64 = 811589153;

impl TryFrom<Vec<i64>> for ProblemInput {
    type Error = anyhow::Error;

    fn try_from(list: Vec<i64>) -> Result<Self, Self::Error> {
        let tmp_head = Node::new(0);
        let mut zero_point = None;

        let mut nodes = vec![];
        for value in list {
            let curr_node = Node::new(value);
            if value == 0 {
                zero_point = Some(Rc::downgrade(&curr_node));
            }
            RefCell::borrow_mut(&tmp_head).insert_before(&curr_node)?;
            nodes.push(curr_node);
        }

        RefCell::borrow_mut(&tmp_head).remove_self()?;
        let list = Rc::downgrade(&nodes[0]);
        Ok(ProblemInput {
            nodes,
            list,
            zero_point: zero_point.context("no zero")?,
        })
    }
}

type Input = Vec<i64>;
type Output = i64;

#[derive(Debug, Clone)]
struct Node {
    prev: Weak<RefCell<Node>>,
    next: Weak<RefCell<Node>>,
    me: Weak<RefCell<Node>>,

    value: i64,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(Self {
                prev: me.clone(),
                next: me.clone(),
                me: me.clone(),
                value,
            })
        })
    }

    fn insert_after(&mut self, node: &Rc<RefCell<Node>>) -> Result<()> {
        if Weak::ptr_eq(&self.me, &self.next) {
            self.next = Rc::downgrade(node);
            self.prev = Rc::downgrade(node);
            let mut next = RefCell::borrow_mut(node);
            next.prev = self.me.clone();
            next.next = self.me.clone();
            return Ok(());
        }

        let old_next = self.next.upgrade().context("reference died")?;
        self.next = RefCell::borrow(node).me.clone();
        RefCell::borrow_mut(node).prev = self.me.clone();
        RefCell::borrow_mut(node).next = RefCell::borrow(&old_next).me.clone();
        RefCell::borrow_mut(&old_next).prev = RefCell::borrow(node).me.clone();

        Ok(())
    }

    fn insert_before(&mut self, node: &Rc<RefCell<Node>>) -> Result<()> {
        if Weak::ptr_eq(&self.me, &self.next) {
            return self.insert_after(node);
        }

        let old_prev = self.prev.upgrade().context("dead pointer")?;
        self.prev = Rc::downgrade(node);

        RefCell::borrow_mut(&old_prev).next = self.prev.clone();
        RefCell::borrow_mut(node).prev = Rc::downgrade(&old_prev);
        RefCell::borrow_mut(node).next = self.me.clone();

        Ok(())
    }

    fn remove_self(&mut self) -> Result<()> {
        ensure!(!Weak::ptr_eq(&self.me, &self.next));

        if Weak::ptr_eq(&self.next, &self.prev) {
            // Only one other element
            let other = self.next.upgrade().context("reference died")?;
            let mut other = RefCell::borrow_mut(&other);
            other.prev = other.me.clone();
            other.next = other.me.clone();
            self.next = self.me.clone();
            self.prev = self.me.clone();
            return Ok(());
        }

        let old_prev = self.prev.upgrade().context("reference died")?;
        let old_next = self.next.upgrade().context("reference died")?;

        self.prev = RefCell::borrow(&old_prev).next.clone();
        self.next = RefCell::borrow(&old_next).prev.clone();

        RefCell::borrow_mut(&old_prev).next = Rc::downgrade(&old_next);
        RefCell::borrow_mut(&old_next).prev = Rc::downgrade(&old_prev);

        Ok(())
    }
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Result<Input> {
    let mut result = vec![];
    for l in input.lines() {
        result.push(l.parse()?);
    }

    Ok(result)
}

#[allow(dead_code)]
fn print_list(head: &Weak<RefCell<Node>>) -> Result<()> {
    let head = head.upgrade().context("dead reference")?;
    print!("[{}", RefCell::borrow(&head).value);

    let mut next = RefCell::borrow(&head)
        .next
        .upgrade()
        .context("dead reference")?;
    while !Rc::ptr_eq(&head, &next) {
        print!(", {}", RefCell::borrow(&next).value);
        let tmp_next = RefCell::borrow(&next)
            .next
            .upgrade()
            .context("dead reference")?;
        next = tmp_next;
    }
    println!("]");
    Ok(())
}

fn step(node: &Rc<RefCell<Node>>, steps: i64, node_count: i64) -> Result<Rc<RefCell<Node>>> {
    if steps == 0 {
        return Ok(node.clone());
    }
    let mut curr = node.clone();
    let (forward, steps) = if steps > 0 {
        (true, steps)
    } else {
        (false, -steps)
    };
    let steps = steps % (node_count);

    for _ in 0..steps {
        let next_cell = if forward {
            curr.borrow().next.clone()
        } else {
            curr.borrow().prev.clone()
        };
        curr = next_cell.upgrade().context("dead reference")?;
    }
    Ok(curr)
}

fn move_node(node: &Rc<RefCell<Node>>, node_count: i64) -> Result<()> {
    let prev = node.borrow().prev.clone();
    node.borrow_mut().remove_self()?;
    let steps = node.borrow().value;
    let prev = step(&prev.upgrade().context("dead node")?, steps, node_count - 1)?;

    prev.borrow_mut().insert_after(node)?;
    Ok(())
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> Result<Output> {
    let input: ProblemInput = input.clone().try_into()?;
    let node_count = input.nodes.len() as i64;
    for n in &input.nodes {
        move_node(n, node_count)?;
    }

    let mut result = 0;
    let curr_node = input.zero_point;
    let curr_node = step(&curr_node.upgrade().context("dead node")?, 1000, node_count)?;
    result += curr_node.borrow().value;
    let curr_node = step(&curr_node, 1000, node_count)?;
    result += curr_node.borrow().value;
    let curr_node = step(&curr_node, 1000, node_count)?;
    result += curr_node.borrow().value;
    Ok(result)
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> Result<Output> {
    let input = input.iter().map(|v| v * DECRYPTION_KEY).collect_vec();
    let input: ProblemInput = input.try_into()?;
    let node_count = input.nodes.len() as i64;

    for _ in 0..10 {
        for n in &input.nodes {
            move_node(n, node_count)?;
        }
    }

    let mut result = 0;
    let curr_node = input.zero_point;
    let curr_node = step(&curr_node.upgrade().context("dead node")?, 1000, node_count)?;
    result += curr_node.borrow().value;
    let curr_node = step(&curr_node, 1000, node_count)?;
    result += curr_node.borrow().value;
    let curr_node = step(&curr_node, 1000, node_count)?;
    result += curr_node.borrow().value;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        let input: ProblemInput = input.try_into()?;
        let node_count = input.nodes.len() as i64;

        print_list(&input.list)?;
        for n in &input.nodes {
            move_node(n, node_count)?;
            print_list(&input.list)?;
        }
        let input = input_generator(INPUT_STR)?;

        assert_eq!(part1(&input)?, 3);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        let input = input.iter().map(|v| v * DECRYPTION_KEY).collect_vec();
        let input: ProblemInput = input.try_into()?;
        let node_count = input.nodes.len() as i64;

        print_list(&input.list)?;
        for round in 0..10 {
            for n in &input.nodes {
                move_node(n, node_count)?;
            }
            println!("\nAfter {} rounds of mixing", round + 1);
            print_list(&input.zero_point)?;
        }
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 1623178306);
        Ok(())
    }
}

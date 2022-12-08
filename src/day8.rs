use anyhow::{ensure, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use salusa_aoc::MatrixTranspose;

type Input = Vec<Vec<u8>>;
type Output = i32;

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Result<Input> {
    let result: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.trim().as_bytes().to_vec())
        .collect();
    let result = result.transpose();
    Ok(result)
}

fn visibility_map(input: &Input) -> Result<Vec<Vec<bool>>> {
    ensure!(!input.is_empty(), "Must have non-zero width");
    let width = input.len();
    let height = input[0].len();

    let mut result = vec![vec![false; height]; width];
    // Doing this in two passes.

    // 1: Top->Bottom & Left->Right
    let mut highest_cols = vec![None; width];
    let mut highest_rows = vec![None; height];

    for (x, vert) in highest_cols.iter_mut().enumerate() {
        for (y, horiz) in highest_rows.iter_mut().enumerate() {
            let curr_height = input[x][y];
            if vert.is_none() || vert.unwrap() < curr_height {
                *vert = Some(curr_height);
                result[x][y] = true;
            }

            if horiz.is_none() || horiz.unwrap() < curr_height {
                *horiz = Some(curr_height);
                result[x][y] = true;
            }
        }
    }

    // 2: Bottom->Top & Right->Left
    let mut highest_cols = vec![None; width];
    let mut highest_rows = vec![None; height];

    for (x, vert) in highest_cols.iter_mut().enumerate().rev() {
        for (y, horiz) in highest_rows.iter_mut().enumerate().rev() {
            let curr_height = input[x][y];
            if vert.is_none() || vert.unwrap() < curr_height {
                *vert = Some(curr_height);
                result[x][y] = true;
            }

            if horiz.is_none() || horiz.unwrap() < curr_height {
                *horiz = Some(curr_height);
                result[x][y] = true;
            }
        }
    }

    Ok(result)
}

#[allow(dead_code)]
fn print_visibility(visibility: &[Vec<bool>]) {
    let height = visibility[0].len();

    for y in 0..height {
        for col in visibility {
            let sym = if col[y] { '#' } else { '.' };
            print!("{}", sym);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_scores(scores: &[Vec<i32>]) {
    let height = scores[0].len();

    for y in 0..height {
        for col in scores {
            print!("{}\t", col[y]);
        }
        println!();
    }
}

fn scenic_scores(trees: &Input) -> Result<Vec<Vec<i32>>> {
    ensure!(!trees.is_empty(), "Must have non-zero width");
    let width = trees.len();
    let height = trees[0].len();

    let mut result = vec![vec![0; height]; width];

    // There has got to be a better way that roughly n^3, but I don't think it's worth figuring out
    for (curr_x, col) in trees.iter().enumerate() {
        if curr_x == 0 || curr_x == width - 1 {
            continue;
        }
        for (curr_y, curr_height) in col.iter().enumerate() {
            if curr_y == 0 || curr_y == height - 1 {
                continue;
            }
            let mut score = 1;

            let mut curr_count = 0;
            for x in (0..curr_x).rev() {
                curr_count += 1;

                if trees[x][curr_y] >= *curr_height {
                    break;
                }
            }
            score *= curr_count;
            curr_count = 0;
            #[allow(clippy::needless_range_loop)]
            for x in curr_x + 1..width {
                curr_count += 1;

                if trees[x][curr_y] >= *curr_height {
                    break;
                }
            }
            score *= curr_count;

            curr_count = 0;
            for y in (0..curr_y).rev() {
                curr_count += 1;

                if trees[curr_x][y] >= *curr_height {
                    break;
                }
            }
            score *= curr_count;
            curr_count = 0;
            for y in curr_y + 1..height {
                curr_count += 1;

                if trees[curr_x][y] >= *curr_height {
                    break;
                }
            }
            score *= curr_count;
            result[curr_x][curr_y] = score;
        }
    }
    Ok(result)
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> Result<Output> {
    let visibility = visibility_map(input)?;
    // print_visibility(&visibility);
    let visibile = visibility
        .iter()
        .map(|col| col.iter().filter(|v| **v).count() as i32)
        .sum();
    Ok(visibile)
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> Result<Output> {
    let scores = scenic_scores(input)?;
    // print_scores(&scores);
    let best_score = *scores
        .iter()
        .map(|col| col.iter().max().unwrap())
        .max()
        .unwrap();

    Ok(best_score)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_STR: &str = "30373
                                25512
                                65332
                                33549
                                35390";

    #[test]
    fn part1_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part1(&input)?, 21);
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = input_generator(INPUT_STR)?;
        assert_eq!(part2(&input)?, 8);
        Ok(())
    }
}

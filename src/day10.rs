use rayon::prelude::*;
use std::collections::HashSet;

pub struct Map {
    heads: Vec<(usize, (usize, usize))>,
    grid: Vec<Vec<usize>>,
    max_col: usize,
    max_row: usize,
}

impl Map {
    fn next_up(&self, pos: &(usize, usize), height: &usize) -> Vec<(usize, (usize, usize))> {
        let mut next = Vec::new();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in directions {
            let x = pos.0 as i32 + dx;
            let y = pos.1 as i32 + dy;
            if x >= 0 && x < self.max_col as i32 && y >= 0 && y < self.max_row as i32 {
                let next_height = self.grid[y as usize][x as usize];
                if next_height == height + 1 {
                    next.push((next_height, (x as usize, y as usize)));
                }
            }
        }

        next
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    let mut heads = Vec::new();
    let max_col = input.lines().next().unwrap().len();
    let mut max_row = 0;

    let grid: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            max_row += 1;
            line.trim()
                .chars()
                .enumerate()
                .map(|(col, ch)| {
                    if ch == '0' {
                        heads.push((0, (col, row)));
                    }
                    ch.to_digit(10).unwrap() as usize
                }).collect()
        }).collect();

    Map { heads, grid, max_col, max_row }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Map) -> usize {
    input.heads
        .par_iter()
        .map(|&head| {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let mut next_pos = vec![head];
            let mut thread_score = 0;

            while !next_pos.is_empty() {
                let (height, pos) = next_pos.pop().unwrap();

                if !visited.contains(&pos) {
                    visited.insert(pos.clone());
                    if height == 9 {
                        thread_score += 1;
                    } else {
                        next_pos.extend(input.next_up(&pos, &height));
                    }
                }
            }

            thread_score
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Map) -> usize {
    input.heads
        .par_iter()
        .map(|&head| {
            let mut next_pos = vec![head];
            let mut thread_score = 0;

            while !next_pos.is_empty() {
                let (height, pos) = next_pos.pop().unwrap();

                if height == 9 {
                    thread_score += 1;
                } else {
                    next_pos.extend(input.next_up(&pos, &height));
                }
            }

            thread_score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "89010123
                        78121874
                        87430965
                        96549874
                        45678903
                        32019012
                        01329801
                        10456732";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 36);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 81);
    }
}
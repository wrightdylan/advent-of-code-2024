use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

const MASK: usize = (1 << 24) - 1;

fn next_secret(sn: usize) -> usize {
    let sn = ((sn << 6) ^ sn) & MASK;
    let sn = ((sn >> 5) ^ sn) & MASK;
    ((sn << 11) ^ sn) & MASK
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_part1(seeds: &Vec<usize>) -> usize {
    seeds.par_iter()
        .map(|&sn| {
            (0..2000).fold(sn, |acc, _| next_secret(acc))
        })
        .sum()
}

// Only one sequence of 4 changes allowed for all sales.
// The main thing to bear in mind is that you add the FIRST value of the FIRST time
// the sequence is encountered, not the best value at any time. This method adds
// values from random sequences, but the most common sequence will always be greatest
#[aoc(day22, part2)]
pub fn solve_part2(seeds: &Vec<usize>) -> usize {
    let totals: Vec<AtomicUsize> = (0..(1 << 20))
        .map(|_| AtomicUsize::new(0))
        .collect();

    seeds.par_iter()
        .for_each(|seed| {
            let mut sn = *seed;
            let mut seen = vec![false; 1 << 20];
            let mut diffs = 0;
            for i in 0..2000 {
                let next_sn = next_secret(sn);
                let ones = next_sn % 10;
                diffs = ((diffs << 5) | (ones + 9 - sn % 10)) & 0xFFFFF;
                sn = next_sn;
                if i >= 3 {
                    if seen[diffs] {
                        continue;
                    }

                    totals[diffs].fetch_add(ones, Ordering::Relaxed);
                    seen[diffs] = true;
                }
            }
        });

    totals.iter()
        .map(|diff_idx| diff_idx.load(Ordering::Relaxed))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "1
10
100
2024";

    const TEST2: &str = "1
2
3
2024";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 37327623);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 23);
    }
}
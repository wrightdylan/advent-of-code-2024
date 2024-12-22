use crate::prelude::*;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_part1(isn: &Vec<usize>) -> usize {
    0
}

// #[aoc(day22, part2)]
// pub fn solve_part2(isn: &Vec<usize>) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "1
10
100
2024";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 37327623);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(solve_part2(&input_generator(TEST)), 61);
    // }
}
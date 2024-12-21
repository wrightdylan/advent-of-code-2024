use crate::prelude::*;

// Keypad 1
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+


// Keypads 2, 3, 4
// +---+---+
// | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<usize>) {
    let mut numbers = Vec::new();

    let codes = input
        .lines()
        .map(|line| {
            numbers.push(line[..3].parse().unwrap());
            line.chars().collect()
        }).collect();

    (codes, numbers)
}

// #[aoc(day21, part1)]
// pub fn solve_part1((codes, numbers): &(Vec<Vec<char>>, Vec<usize>)) -> usize {
//     0
// }

// #[aoc(day21, part2)]
// pub fn solve_part2((codes, numbers): &(Vec<Vec<char>>, Vec<usize>)) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "029A
980A
179A
456A
379A";

    // #[test]
    // fn part1_test() {
    //     assert_eq!(solve_part1(&input_generator(TEST)), 126384);
    // }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(solve_part2(&input_generator(TEST)), 61);
    // }
}
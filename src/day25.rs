use crate::prelude::*;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> usize {
    0
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &usize) -> usize {
    0
}

// #[aoc(day25, part2)]
// pub fn solve_part2(input: &usize) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 22);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(solve_part2(&input_generator(TEST)), 61);
    // }
}
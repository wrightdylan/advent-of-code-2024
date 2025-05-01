#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .split("\n\n")
        .map(|block| block.bytes()
            .filter(|&byte| byte != b'\n')
            .fold(0, |acc, byte| acc << 1 | (byte == b'#') as u64))
        .partition(|state| state & 1 == 1)
}

#[aoc(day25, part1)]
pub fn solve_part1((locks, keys): &(Vec<u64>, Vec<u64>)) -> usize {
    locks
        .iter()
        .flat_map(|&lock| keys.iter().map(move |&key| lock & key))
        .filter(|&result| result == 0)
        .count()
}

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
        assert_eq!(solve_part1(&input_generator(TEST)), 3);
    }
}
#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.trim().split_once(": ").unwrap();
            (
                left.parse().unwrap(),
                right.split_whitespace().map(|num| num.parse().unwrap()).collect()
            )
        }).collect()
}

fn validator(nums: &Vec<usize>, idx: usize, current: usize, test: &usize, part2: bool) -> bool {
    if idx >= nums.len() {
        return current == *test;
    }

    if validator(nums, idx + 1, current + nums[idx], test, part2) {
        return true;
    }
    if validator(nums, idx + 1, current * nums[idx], test, part2) {
        return true;
    }
    if part2 {
        let combined = current * 10usize.pow((nums[idx] as f64).log10() as u32 + 1) + nums[idx];
        if validator(nums, idx + 1, combined, test, part2) {
            return true;
        }
    }

    false
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;

    for (test, nums) in input.iter() {
        if validator(nums, 1, nums[0], test, false) {
            result += test;
        }
    }

    result
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;

    for (test, nums) in input.iter() {
        if validator(nums, 1, nums[0], test, true) {
            result += test;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 3749);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 11387);
    }
}
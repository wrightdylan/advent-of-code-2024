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

fn calibration(nums: &Vec<usize>, idx: usize, current: usize, results: &mut Vec<usize>, part2: bool) {
    if idx >= nums.len() {
        results.push(current);
        return;
    }

    calibration(nums, idx + 1, current + nums[idx], results, part2);
    calibration(nums, idx + 1, current * nums[idx], results, part2);
    if part2 {
        let combined = format!("{}{}", current.to_string(), nums[idx].to_string()).parse().unwrap();
        calibration(nums, idx + 1, combined, results, part2);
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;

    for (test, nums) in input.iter() {
        let mut results = Vec::new();
        calibration(nums, 1, nums[0], &mut results, false);

        if results.contains(test) {
            result += test;
        }
    }

    result
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;

    for (test, nums) in input.iter() {
        let mut results = Vec::new();
        calibration(nums, 1, nums[0], &mut results, true);

        if results.contains(test) {
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
use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split_whitespace().map(|num| num.parse().unwrap()).collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    input.iter().for_each(|&num| *stones.entry(num).or_insert(0) += 1);
    
    for _ in 0..25 {
        let mut new_map: HashMap<usize, usize> = HashMap::new();
        for (&key, &val) in stones.iter() {
            if key == 0 {
                *new_map.entry(1).or_insert(0) += val;
            } else {
                let digit_count = (key as f64).log10() as u32 + 1;
                if digit_count % 2 == 0 {
                    let divisor = 10_usize.pow(digit_count / 2);
                    *new_map.entry(key / divisor).or_insert(0) += val;
                    *new_map.entry(key % divisor).or_insert(0) += val;
                } else {
                    *new_map.entry(key * 2024).or_insert(0) += val;
                }
            }
        }
        stones = new_map;
    }

    stones.iter().map(|(_, val)| val).sum()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    input.iter().for_each(|&num| *stones.entry(num).or_insert(0) += 1);
    
    for _ in 0..75 {
        let mut new_map: HashMap<usize, usize> = HashMap::new();
        for (&key, &val) in stones.iter() {
            if key == 0 {
                *new_map.entry(1).or_insert(0) += val;
            } else {
                let digit_count = (key as f64).log10() as u32 + 1;
                if digit_count % 2 == 0 {
                    let divisor = 10_usize.pow(digit_count / 2);
                    *new_map.entry(key / divisor).or_insert(0) += val;
                    *new_map.entry(key % divisor).or_insert(0) += val;
                } else {
                    *new_map.entry(key * 2024).or_insert(0) += val;
                }
            }
        }
        stones = new_map;
    }

    stones.iter().map(|(_, val)| val).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "125 17";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 55312);
    }
}
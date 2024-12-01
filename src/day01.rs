use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();

        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }

    (left, right)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut left = input.0.clone();
    let mut right = input.1.clone();
    left.sort();
    right.sort();

    left.iter().zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2((left, right): &(Vec<usize>, Vec<usize>)) -> usize {
    // I was hoping caching would boost performance, but the number of unique lookups reduced performance
    // let mut cache: HashMap<usize, usize> = HashMap::with_capacity(left.len());

    left.iter()
        .map(|&val| {
            // *cache.entry(val).or_insert_with(|| {
                let count = right.iter().filter(|&&x| x == val).count();
                count * val
            // })
        }).sum()
}

#[aoc(day1, part2, Cached)]
pub fn solve_part2_cached((left, right): &(Vec<usize>, Vec<usize>)) -> usize {
    // On second though, I may have been chaching incorrectly
    let mut cache: HashMap<usize, usize> = right.iter()
        .fold(HashMap::new(), |mut acc, &val| {
            *acc.entry(val).or_insert(0) += 1;
            acc
        });
    
    cache.iter_mut()
        .for_each(|(key, val)| {
            let product = *key * *val;
            *val = product;
        });

    left.iter()
        .map(|&val| {
            cache.get(&val).unwrap_or(&0)
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "3   4
                        4   3
                        2   5
                        1   3
                        3   9
                        3   3";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(&TEST)), 11);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(&TEST)), 31);
    }
}
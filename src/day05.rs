use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

type Order = HashMap<usize, HashSet<usize>>;
type Updates = Vec<Vec<usize>>;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Order, Updates) {
    let (input_ordering, input_updates) = input.split_once("\n\n").unwrap();

    (
        input_ordering
            .lines()
            .fold(HashMap::new(), |mut map, order| {
                let (first, second) = order.trim().split_once('|').unwrap();
                map.entry(first.parse().unwrap())
                    .or_insert(HashSet::new())
                    .insert(second.parse().unwrap());
                map
            }),
        input_updates
            .lines()
            .map(|update| update.trim().split(',').map(|num| num.parse().unwrap()).collect())
            .collect()
    )
}

// reorders an incorrectly ordered update and returns the middle number
// I was going to use a decision tree, but then I realised the orders could simply be sorted
fn reorder(ordering: &Order, update: &Vec<usize>) -> usize {
    let mut sorted = update.clone();

    sorted.sort_by(|a, b| {
        if let Some(b_vals) = ordering.get(b) {
            if b_vals.contains(a) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        }
    });
    sorted[update.len() / 2]
}

#[aoc(day5, part1)]
pub fn solve_part1((ordering, updates): &(Order, Updates)) -> usize {
    let mut acc = 0;

    for update in updates {
        let mut valid = true;

        'outer: for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if let Some(values) = ordering.get(&update[i]) {
                    if !values.contains(&update[j]) {
                        valid = false;
                        break 'outer;
                    }
                } else {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            acc += update[update.len() / 2];
        }
    }
    
    acc
}

#[aoc(day5, part2)]
pub fn solve_part2((ordering, updates): &(Order, Updates)) -> usize {
    let mut acc = 0;

    for update in updates {
        let mut valid = true;

        'outer: for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if let Some(values) = ordering.get(&update[i]) {
                    if !values.contains(&update[j]) {
                        valid = false;
                        break 'outer;
                    }
                } else {
                    valid = false;
                    break;
                }
            }
        }

        if !valid {
            acc += reorder(ordering, update);
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 143);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 123);
    }
}
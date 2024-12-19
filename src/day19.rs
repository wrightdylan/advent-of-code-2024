use crate::prelude::*;
// use trie_rs::{map::Trie, map::TrieBuilder, inc_search::Answer};

// Pretty similar to what I was trying to do with the trie, minus the trie
fn search_pattern<'a>(pattern: &'a str, valid: &Vec<&'a str>, cache: &mut HashMap<&'a str, usize>, max_len: usize) -> usize {
    let mut combos = 0;
    if pattern.is_empty() {
        return 1;
    }
    if cache.contains_key(pattern) {
        return *cache.get(pattern).unwrap();
    }
    

    for i in 1..=max_len.min(pattern.len()) {
        if valid.contains(&&pattern[..i]) {
            let subcount = search_pattern(&pattern[i..], valid, cache, max_len);
            combos += subcount;
        }
    }
    cache.insert(pattern, combos);
    combos
}

// #[aoc_generator(day19)]
// pub fn input_generator(input: &str) -> (Vec<String>, Vec<String>) {
//     let (towels, patterns) = input.split_once("\n\n").unwrap();

//     (
//         towels.split(", ").map(|part| part.to_string()).collect(),
//         patterns.lines().map(|line| line.to_string()).collect(),
//     )
// }

fn parse_input<'a>(input: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    
    (towels.split(", ").collect(), patterns.lines().collect())
}

// Can't use lifetimes across aoc macros
#[aoc(day19, part1)]
pub fn part1<'a>(input: &str) -> usize {
    let (towels, patterns) = parse_input(input);
    let mut cache = HashMap::new();
    let max_len = towels.iter().map(|towel| towel.len()).max().unwrap_or(0);

    patterns
        .iter()
        .filter(|pattern| search_pattern(pattern, &towels, &mut cache, max_len) > 0)
        .count()
}

// Test OK, 509085811057 too low
#[aoc(day19, part2)]
pub fn part2<'a>(input: &str) -> usize {
    let (towels, patterns) = parse_input(input);
    let mut cache = HashMap::new();
    let max_len = towels.iter().map(|towel| towel.len()).max().unwrap_or(0);

    patterns.iter()
        .map(|pattern| search_pattern(pattern, &towels, &mut cache, max_len))
        .filter(|&n| n > 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";



    #[test]
    fn part1_test() {
        assert_eq!(part1(&TEST), 6);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&TEST), 16);
    }
}
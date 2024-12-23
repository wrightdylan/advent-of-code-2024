use crate::prelude::*;

pub struct Node {
    name: String,
    edges: Vec<Edge>,
}

pub struct Edge {
    nodes: [String; 2],
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> usize {
    for line in input.lines() {}
    0
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &usize) -> usize {
    0
}

// #[aoc(day23, part2)]
// pub fn solve_part2(input: &usize) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 7);
    }

    // #[test]
    // fn part2_test() {
    //     assert_eq!(solve_part2(&input_generator(TEST)), 61);
    // }
}
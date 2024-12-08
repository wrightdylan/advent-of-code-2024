use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn valid_antinodes(a: &(isize, isize), b: &(isize, isize), bounds: &(isize, isize)) -> Vec<(isize, isize)>{
    let mut antinodes = Vec::new();

    let dx = b.0 - a.0;
    let dy = b.1 - a.1;

    let an1 = (a.0 - dx, a.1 - dy);
    let an2 = (b.0 + dx, b.1 + dy);

    if an1.0 >= 0 && an1.0 < bounds.0 && an1.1 >= 0 && an1.1 < bounds.1 {
        antinodes.push(an1);
    }
    if an2.0 >= 0 && an2.0 < bounds.0 && an2.1 >= 0 && an2.1 < bounds.1 {
        antinodes.push(an2);
    }

    antinodes
}

fn valid_harmonics(a: &(isize, isize), b: &(isize, isize), bounds: &(isize, isize)) -> Vec<(isize, isize)>{
    let mut harmonics = Vec::new();

    let mut x1 = a.0;
    let mut y1 = a.1;

    let mut x2 = b.0;
    let mut y2 = b.1;


    let dx = b.0 - a.0;
    let dy = b.1 - a.1;

    while x1 >= 0 && x1 < bounds.0 && y1 >= 0 && y1 < bounds.1 {
        harmonics.push((x1, y1));
        x1 -= dx;
        y1 -= dy;

    }
    while x2 >= 0 && x2 < bounds.0 && y2 >= 0 && y2 < bounds.1 {
        harmonics.push((x2, y2));
        x2 += dx;
        y2 += dy;
    }

    harmonics
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> (HashMap<char, Vec<(isize, isize)>>, (isize, isize)) {
    let mut antennae = HashMap::new();
    let mut max_row = 0;
    
    input
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            max_row += 1;
            line.trim()
                .chars()
                .enumerate()
                .for_each(|(col, ch)| {
                    if ch != '.' {
                        antennae
                            .entry(ch)
                            .or_insert(Vec::new())
                            .push((col as isize, row as isize));
                    }
                })
        });

    (antennae, (input.lines().next().unwrap().len() as isize, max_row))
}

#[aoc(day8, part1)]
pub fn solve_part1(
    (antennae, bounds): &(
        HashMap<char, Vec<(isize, isize)>>,
        (isize, isize),
    ),
) -> usize {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    
    for (_, pos) in antennae {
        let pairs: Vec<((isize, isize), (isize, isize))> = pos.iter()
            .copied()
            .combinations(2)
            .map(|pair| (pair[0], pair[1]))
            .collect();

        for (a, b) in pairs {
            antinodes.extend(valid_antinodes(&a, &b, bounds).into_iter());
        }
    }
    
    antinodes.len()
}


#[aoc(day8, part2)]
pub fn solve_part2(
    (antennae, bounds): &(
        HashMap<char, Vec<(isize, isize)>>,
        (isize, isize),
    ),
) -> usize {
    let mut harmonics: HashSet<(isize, isize)> = HashSet::new();
    
    for (_, pos) in antennae {
        let pairs: Vec<((isize, isize), (isize, isize))> = pos.iter()
            .copied()
            .combinations(2)
            .map(|pair| (pair[0], pair[1]))
            .collect();

        for (a, b) in pairs {
            harmonics.extend(valid_harmonics(&a, &b, bounds).into_iter());
        }
    }

    harmonics.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

const MINI: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 14);
    }

    #[test]
    fn part2_test_mini() {
        assert_eq!(solve_part2(&input_generator(MINI)), 9);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 34);
    }
}
use crate::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Track {
    Path,
    Wall,
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> (Grid<Track>, (usize, usize)) {
    let mut entity = Vec::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut start = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let item = match ch {
                'E' => Track::Path,
                '.' => Track::Path,
                'S' => { start = (col, row); Track::Path },
                '#' => Track::Wall,
                _ => unreachable!(),
            };
            entity.push(item);
        }
    }

    (Grid::new(width, height, entity), start)
}

#[aoc(day20, part1)]
pub fn solve_part1((track, start): &(Grid<Track>, (usize, usize))) -> usize {
    let mut course = HashMap::new();
    let mut race = Vec::from([start.clone()]);
    let mut poss_sc = Vec::new();
    let mut idx = 0;

    while let Some(pos) = race.pop() {
        for point in track.neighbours_as(&pos, Track::Path) {
            if !course.contains_key(&point) {
                race.push(point);
            }
        }
        course.insert(pos, idx as usize);
        idx += 1;

        for (dy, dx) in &ORTHO {
            let tiles = track.look(&pos, &(*dx, *dy), 2);
            if tiles.len() == 2 {
                if tiles[0].1 == Track::Wall && matches!(tiles[1].1, Track::Path) {
                        poss_sc.push((pos, tiles[1].0));
                }
            }
        }
    }

    // Tally cheats
    // let mut cheats = HashMap::new();
    let mut cheat_total = 0;

    for cheat in poss_sc {
        let start = course.get(&cheat.0).unwrap();
        let end = course.get(&cheat.1).unwrap();
        if end > start {
            // *cheats.entry(end - start - 2).or_insert(0) += 1;
            if (end - start - 2) >= 100 {
                cheat_total += 1;
            }
        }
    }

    // let mut sorted_keys: Vec<&usize> = cheats.keys().collect();
    // sorted_keys.sort();

    // for key in sorted_keys {
    //     if let Some(value) = cheats.get(key) {
    //         println!("There are {} cheat(s) that save {} picoseconds.", value, key);
    //     }
    // }

    cheat_total
}

#[aoc(day20, part2)]
pub fn solve_part2((track, start): &(Grid<Track>, (usize, usize))) -> usize {
    let mut course = HashMap::new();
    let mut race = Vec::from([start.clone()]);
    let mut poss_sc = Vec::new();
    let mut idx = 0;

    while let Some(pos) = race.pop() {
        for point in track.neighbours_as(&pos, Track::Path) {
            if !course.contains_key(&point) {
                race.push(point);
            }
        }
        course.insert(pos, idx as usize);
        idx += 1;

        let tiles = track.in_range_as(&pos, 20, Track::Path);
        poss_sc.push((pos, tiles));
    }

    poss_sc.par_iter()
        .map(|(pos, sc)| {
            let start = course.get(&pos).unwrap();
            let mut count = 0;
            for (cheat, md) in sc {
                let end = course.get(&cheat).unwrap();
                if end > start && (*end as i32 - *start as i32 - *md as i32) >= 100 {
                    count += 1;
                }
            }
            count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 10);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 10);
    }
}
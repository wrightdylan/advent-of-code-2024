use std::collections::{HashSet, VecDeque};

pub struct Map {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

// Voronoi cells
#[derive(Debug)]
pub struct Region {
    plant_type: char,
    coords: HashSet<(usize, usize)>,
    area: usize,
}

fn voronoi_tesselation(grid: &Vec<Vec<char>>, rows: usize, cols: usize) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // I'll keep things row-major for consistency
    for row in 0..rows {
        for col in 0..cols {
            if visited.contains(&(row, col)) {
                continue;
            }

            let plant_type = grid[row][col];
            let mut coords = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((row, col));

            // Find all coordinates in the region, by region
            while let Some((r, c)) = queue.pop_front() {
                if visited.contains(&(r, c)) {
                    continue;
                }

                visited.insert((r, c));
                coords.insert((r, c));

                for (dr, dc) in &directions {
                    let new_r = (r as i32 + dr) as usize;
                    let new_c = (c as i32 + dc) as usize;

                    if new_r < rows && new_c < cols &&
                       grid[new_r][new_c] == plant_type &&
                       !visited.contains(&(new_r, new_c)) {
                        queue.push_back((new_r, new_c));
                       }
                }
            }

            regions.push(Region {
                plant_type,
                area: coords.len(),
                coords,
            });
        }
    }

    regions
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> (Vec<Region>, Map) {
    let max_col = input.lines().next().unwrap().len();
    let mut max_row = 0;

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            max_row += 1;
            line.chars()
                .collect()
        }).collect();

    (
        voronoi_tesselation(&grid, max_row, max_col),
        Map {
            grid,
            rows: max_row,
            cols: max_col,
        },
    )
}

#[aoc(day12, part1)]
pub fn solve_part1((regions, map): &(Vec<Region>, Map)) -> usize {
    let mut total = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for region in regions {
        let mut perimeter = 0;
        for &(r, c) in &region.coords {
            for (dr, dc) in &directions {
                let new_r = (r as i32 + dr) as usize;
                let new_c = (c as i32 + dc) as usize;

                if new_r >= map.rows || new_c >= map.cols ||
                    map.grid[new_r][new_c] != region.plant_type {
                    perimeter += 1;
                    }
            }
        }

        total += region.coords.len() * perimeter;
    }

    total
}

#[aoc(day12, part2)]
pub fn solve_part2((regions, map): &(Vec<Region>, Map)) -> usize {
    let mut total = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for region in regions {
        let mut edges = HashSet::new();
        for &(r, c) in &region.coords {
            for (idx, (dr, dc)) in directions.iter().enumerate() {
                let new_r = (r as i32 + dr) as usize;
                let new_c = (c as i32 + dc) as usize;

                if new_r >= map.rows || new_c >= map.cols ||
                   map.grid[new_r][new_c] != region.plant_type {
                    edges.insert((r, c, idx));
                }
            }
        }

        let mut sides = 0;
        let mut visited_edges = HashSet::new();

        for &(r, c, dir) in &edges {
            if visited_edges.contains(&(r, c, dir)) {
                continue;
            }

            sides += 1;

            let perp_dirs = match dir {
                0 | 2 => [(1, 0), (-1, 0)],
                1 | 3 => [(0, 1), (0, -1)],
                _ => unreachable!(),
            };

            for &(dr, dc) in &perp_dirs {
                let mut curr_r = r as i32;
                let mut curr_c = c as i32;

                loop {
                    curr_r += dr;
                    curr_c += dc;

                    if curr_r < 0 || curr_r >= map.rows as i32 ||
                       curr_c < 0 || curr_c >= map.cols as i32 {
                        break;
                    }

                    let curr_coord = (curr_r as usize, curr_c as usize, dir);
                    if !edges.contains(&curr_coord) {
                        break;
                    }

                    visited_edges.insert(curr_coord);
                }
            }
        }
        
        total += region.area * sides;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "AAAA
BBCD
BBCC
EEEC";

    const TEST2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const TEST5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 140);
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 772);
    }

    #[test]
    fn part1_test3() {
        assert_eq!(solve_part1(&input_generator(TEST3)), 1930);
    }

    #[test]
    fn part2_test1() {
        assert_eq!(solve_part2(&input_generator(TEST1)), 80);
    }

    #[test]
    fn part2_test2() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 436);
    }

    #[test]
    fn part2_test3() {
        assert_eq!(solve_part2(&input_generator(TEST3)), 1206);
    }

    #[test]
    fn part2_test4() {
        assert_eq!(solve_part2(&input_generator(TEST4)), 236);
    }

    #[test]
    fn part2_test5() {
        assert_eq!(solve_part2(&input_generator(TEST5)), 368);
    }
}
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Map {
    Box,
    LBox,
    RBox,
    Floor,
    Wall,
}

pub struct Robot {
    x: usize,
    y: usize,
}

impl Robot {
    fn new(pos: &(usize, usize)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }

    fn step(&mut self, dir: &(i32, i32)) {
        self.x = (self.x as i32 + dir.0) as usize;
        self.y = (self.y as i32 + dir.1) as usize;
    }

    fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

fn offset(from: &(usize, usize), dir: &(i32, i32)) -> (usize, usize) {
    ((from.0 as i32 + dir.0) as usize, (from.1 as i32 + dir.1) as usize)
}

// fn draw_map(map: &Grid<Map>, robot: &Robot) {
//     println!("Width: {}, height: {}", map.width, map.height);
//     for row in 0..map.height {
//         for col in 0..map.width {
//             let ch = match map[(col, row)] {
//                 Map::Box => 'O',
//                 Map::LBox => '[',
//                 Map::RBox => ']',
//                 Map::Floor => '.',
//                 Map::Wall => '#',
//             };
//             if col == robot.x && row == robot.y {
//                 print!("@");
//             } else {
//                 print!("{ch}");
//             }
//         }
//         println!()
//     }
// }

fn moveable(map: &Grid<Map>, dir: &(i32, i32), pos: (usize, usize)) -> bool {
    if let Ok(tile) = map.peek(&pos, dir) {
        let next = offset(&pos, dir);
        match tile {
            Map::Floor => return true,
            Map::Wall => return false,
            Map::Box => return true,
            Map::LBox => {
                return moveable(map, dir, (next.0, next.1)) && moveable(map, dir, (next.0 + 1, next.1))
            },
            Map::RBox => {
                return moveable(map, dir, (next.0 - 1, next.1)) && moveable(map, dir, (next.0, next.1))
            },
        }
    }

    false
}

// North and south only
fn get_group(map: &Grid<Map>, dir: &(i32, i32), pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut group = HashSet::new();

    match map[pos] {
        Map::LBox => {
            let right = (pos.0 + 1, pos.1);
            group.insert(pos);
            group.insert(right);
            group.extend(get_group(map, dir, offset(&pos, dir)));
            group.extend(get_group(map, dir, offset(&right, dir)));
        },
        Map::RBox => {
            let left = (pos.0 - 1, pos.1);
            group.insert(pos);
            group.insert(left);
            group.extend(get_group(map, dir, offset(&pos, dir)));
            group.extend(get_group(map, dir, offset(&left, dir)));
        },
        _ => {},
    };

    return group
}

fn rebundle_positions(hash_set: &HashSet<(usize, usize)>, direction: &(i32, i32)) -> Vec<Vec<(usize, usize)>> {
    let mut sorted_vectors = Vec::new();
    let mut y_to_coords = HashMap::new();

    for &(x, y) in hash_set.iter() {
        y_to_coords.entry(y).or_insert(Vec::new()).push((x, y));
    }

    let mut sorted_y_values: Vec<usize> = y_to_coords.keys().cloned().collect();
    sorted_y_values.sort_by(|a, b| {
        if direction.1 == -1 {
            a.cmp(b)
        } else {
            b.cmp(a)
        }
    });

    for y in sorted_y_values {
        if let Some(coords) = y_to_coords.get(&y) {
            sorted_vectors.push(coords.clone());
        }
    }

    sorted_vectors
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> (Grid<Map>, (usize, usize), Vec<(i32, i32)>) {
    let (map_str, move_str) = input.split_once("\n\n").unwrap();
    let width = map_str.lines().next().unwrap().len();
    let height = map_str.lines().count();

    let mut entity = Vec::new();
    let mut start = (0, 0);

    for (row, line) in map_str.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let map_item = match ch {
                'O' => Map::Box,
                '.' => Map::Floor,
                '@' => { start = (col, row); Map::Floor },
                '#' => Map::Wall,
                _ => unreachable!(),
            };
            entity.push(map_item);
        }
    }

    let grid = Grid::new(width, height, entity);

    let moves = move_str.lines().flat_map(|line| {
        line.chars().map(|ch| match ch {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => unreachable!(),
        })
    }).collect();
    
    (grid, start, moves)
}

#[aoc(day15, part1)]
pub fn solve_part1(
    (grid, start, moves): &(
    Grid<Map>, (usize, usize), Vec<(i32, i32)>
)) -> usize {
    let mut map = Grid::new(grid.width, grid.height, grid.entity.clone());

    for (idx, value) in grid.entity.iter().enumerate() {
        map.entity[idx] = *value;
    }
    let mut robot = Robot::new(start);

    for m in moves {
        match map.peek(&robot.pos(), &m) {
            Ok(tile) => {
                match tile {
                    Map::Box   => {
                        let mut current_pos = robot.pos();
                        let mut boxes = Vec::new();

                        while let Ok(tile) = map.peek(&current_pos, m) {
                            match tile {
                                Map::Box => {
                                    boxes.push(((offset(&current_pos, m)), tile));
                                    current_pos = offset(&current_pos, m);
                                },
                                Map::Floor => {
                                    boxes.push(((offset(&current_pos, m)), tile));
                                    break;
                                },
                                Map::Wall => break,
                                _ => unimplemented!(),
                            }
                        }
                        if let Some(end) = boxes.pop() {
                            if end.1 == Map::Floor {
                                for b in boxes.iter().rev() {
                                    map.slide(b.0, m.clone(), Some(Map::Floor)).ok();
                                }
                                robot.step(m);
                            }
                        }
                    },
                    Map::Floor => robot.step(m),
                    Map::Wall  => continue,
                    _ => unimplemented!(),
                }
            },
            Err(_) => continue,
        }
    }

    let mut score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map[(x, y)] == Map::Box {
                score += 100 * y + x;
            }
        }
    }

    score
}

#[aoc(day15, part2)]
pub fn solve_part2(
    (grid, start, moves): &(
    Grid<Map>, (usize, usize), Vec<(i32, i32)>
)) -> usize {
    let mut new_entity = Vec::new();

    for value in grid.entity.iter() {
        match value {
            Map::Wall | Map::Floor => {
                new_entity.push(*value);
                new_entity.push(*value);
            },
            Map::Box => {
                new_entity.push(Map::LBox);
                new_entity.push(Map::RBox);
            },
            _ => unimplemented!(),
        }
    }

    let mut map = Grid::new(grid.width * 2, grid.height, new_entity);
    let mut robot = Robot::new(&(start.0 * 2, start.1));

    for m in moves {
        match map.peek(&robot.pos(), &m) {
            Ok(tile) => {
                match tile {
                    Map::LBox | Map::RBox   => {
                        if m == &(-1, 0) || m == &(1, 0) { // Left-right
                            let mut current_pos = robot.pos();
                            let mut boxes = Vec::new();

                            while let Ok(tile) = map.peek(&current_pos, m) {
                                match tile {
                                    Map::LBox | Map::RBox => {
                                        boxes.push(((offset(&current_pos, m)), tile));
                                        current_pos = offset(&current_pos, m);
                                    },
                                    Map::Floor => {
                                        boxes.push(((offset(&current_pos, m)), tile));
                                        break;
                                    },
                                    Map::Wall => break,
                                    _ => unimplemented!(),
                                }
                            }
                            if let Some(end) = boxes.pop() {
                                if end.1 == Map::Floor {
                                    for b in boxes.iter().rev() {
                                        map.slide(b.0, m.clone(), Some(Map::Floor)).ok();
                                    }
                                    robot.step(m);
                                }
                            }
                        } else { // Up-down
                            if moveable(&map, m, robot.pos()) {
                                let group = get_group(&map, m, offset(&robot.pos(), m));
                                let bundle = rebundle_positions(&group, m);
                                for row in bundle {
                                    for pos in row {
                                        map.slide(pos, m.clone(), Some(Map::Floor)).ok();
                                    }
                                }
                                robot.step(m);
                            }
                        }
                    },
                    Map::Floor => robot.step(m),
                    Map::Wall  => continue,
                    _ => unimplemented!(),
                }
            },
            Err(_) => continue,
        }
        // draw_map(&map, &robot);
    }
    
    let mut score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map[(x, y)] == Map::LBox {
                score += 100 * y + x;
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const TEST3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 2028);
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 10092);
    }

    #[test]
    fn part2_test2() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 9021);
    }

    #[test]
    fn part2_test3() {
        assert_eq!(solve_part2(&input_generator(TEST3)), 618);
    }
}
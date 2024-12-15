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

fn draw_map(map: &Grid<Map>) {
    println!("Width: {}, height: {}", map.width, map.height);
    for row in 0..map.height {
        for col in 0..map.width {
            let ch = match map[(col, row)] {
                Map::Box => 'O',
                Map::LBox => '[',
                Map::RBox => ']',
                Map::Floor => '.',
                Map::Wall => '#',
            };
            print!("{ch}");
        }
        println!()
    }
}

// fn moveable(map: &Grid<Map>, dir: &(i32, i32), pos: (usize, usize)) -> Option<Vec<HashSet<(usize, usize)>>> {
    
// }

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

    // draw_map(&map);

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
                            todo!()
                            // if let Some(boxes) = moveable(&map, m, offset(&robot.pos(), m)) {
                            //     println!("{:?}", boxes);
                            // }
                        }
                    },
                    Map::Floor => robot.step(m),
                    Map::Wall  => continue,
                    _ => unimplemented!(),
                }
            },
            Err(_) => continue,
        }
        // draw_map(&map);
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
        assert_eq!(solve_part2(&input_generator(TEST3)), 105);
    }
}
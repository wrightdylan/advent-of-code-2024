use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn right(&mut self) {
        *self = match  * self {
            Dir::North => Dir::East,
            Dir::East  => Dir::South,
            Dir::South => Dir::West,
            Dir::West  => Dir::North,
        }
    }
}

#[derive(Debug)]
enum MapError {
    Obstacle,
    Bounds,
}

struct Guard {
    position: (usize, usize),
    bounds: (usize, usize),
    direction: Dir,
    visited: HashSet<(Dir, (usize, usize))>
}

impl Guard {
    fn new(start: (usize, usize), bounds: (usize, usize)) -> Self {
        Self { position: start, bounds, direction: Dir::North, visited: HashSet::from([(Dir::North, (start))]) }
    }

    // fn scout(guard: &Guard) -> Self {
    //     let mut new_dir = guard.direction.clone();
    //     new_dir.right();
    
    //     Self { position: guard.position.clone(), bounds: guard.bounds.clone(), direction: new_dir, visited: HashSet::from([(new_dir, guard.position.clone())]) }
    // }


    // Glance to the right to check if a loop path is available
    // fn glance(&self, obstacles: &HashSet<(usize, usize)>) -> bool {
    //     // let mut scout = Guard::scout(self);
    //     let mut scout = Guard::new(self.position, self.bounds);
    //     scout.direction = self.direction;
    //     // let mut scout_visited = self.visited.clone();
    //     let mut scout_visited = HashSet::new();
    //     scout_visited.insert((scout.direction, scout.position));
    //     scout.direction.right();

    //     loop {
    //         match scout.look(obstacles) {
    //             Ok(()) => {
    //                     scout_visited.insert((scout.direction, scout.position));
    //                     scout.step();
    //                 },
    //             Err(MapError::Obstacle) => {
    //                     scout_visited.insert((scout.direction, scout.position));
    //                     scout.direction.right();
    //                 },
    //             Err(MapError::Bounds) => return false,
    //         }
    //         if scout_visited.contains(&(scout.direction, scout.position)) {
    //             println!("LOOP!!");
    //             let similar: HashSet<_> = self.visited.intersection(&scout_visited).collect();
    //             if similar.len() > 0 {
    //                 println!("{}", similar.len());
    //                 return true;
    //             } else {
    //                 return false;
    //             }
    //         }
    //     }
    // }
    fn test_loop(&self, obstacles: &HashSet<(usize, usize)>) -> bool {
        // let mut scout = Guard::scout(self);
        let mut scout = Guard::new(self.position, self.bounds);
        scout.direction = self.direction;
        // let mut scout_visited = self.visited.clone();
        let mut scout_visited = HashSet::new();
        // scout_visited.insert((scout.direction, scout.position));
        scout.direction.right();
        let start = (scout.direction, scout.position);
        let mut last_move = "step";


        loop {
            match scout.look(obstacles) {
                Ok(_) => {
                        scout_visited.insert((scout.direction, scout.position));
                        scout.step();
                        last_move = "step";
                    },
                Err(MapError::Obstacle) => {
                        if last_move == "turn" {
                            return false;
                        }
                        scout_visited.insert((scout.direction, scout.position));
                        scout.direction.right();
                        last_move = "turn";
                    },
                Err(MapError::Bounds) => return false,
            }
            if scout.direction == start.0 && scout.position == start.1 {
                println!("LOOP!!");
                let similar: HashSet<_> = self.visited.intersection(&scout_visited).collect();
                if similar.len() > 0 {
                    println!("{}", similar.len());
                    return true;
                } else {
                    return false;
                }
            }
            if scout_visited.contains(&(scout.direction, scout.position)) {
                println!("PROBLEM LOOP!!");
                return false;
            }
        }
    }

    // Look one position ahead return Ok if clear, or specific error
    fn look(&self, obstacles: &HashSet<(usize, usize)>) -> Result<(usize, usize), MapError> {
        match self.direction {
            Dir::North => {
                if self.position.1 == 0 {
                    return Err(MapError::Bounds);
                } else if obstacles.contains(&(self.position.0, self.position.1 - 1)) {
                    return Err(MapError::Obstacle);
                } else {
                    Ok((self.position.0, self.position.1 - 1))
                }
            },
            Dir::East  => {
                if self.position.0 == self.bounds.0 {
                    return Err(MapError::Bounds);
                } else if obstacles.contains(&(self.position.0 + 1, self.position.1)) {
                    return Err(MapError::Obstacle);
                } else {
                    Ok((self.position.0 + 1, self.position.1))
                }
            },
            Dir::South => {
                if self.position.1 == self.bounds.1 {
                    return Err(MapError::Bounds);
                } else if obstacles.contains(&(self.position.0, self.position.1 + 1)) {
                    return Err(MapError::Obstacle);
                } else {
                    Ok((self.position.0, self.position.1 + 1))
                }
            },
            Dir::West  => {
                if self.position.0 == 0 {
                    return Err(MapError::Bounds);
                } else if obstacles.contains(&(self.position.0 - 1, self.position.1)) {
                    return Err(MapError::Obstacle);
                } else {
                    Ok((self.position.0 - 1, self.position.1))
                }
            },
        }
    }

    fn step(&mut self) {
        let new_pos = match self.direction {
            Dir::North => (self.position.0, self.position.1 - 1),
            Dir::East  => (self.position.0 + 1, self.position.1),
            Dir::South => (self.position.0, self.position.1 + 1),
            Dir::West  => (self.position.0 - 1, self.position.1),
        };
        self.position = new_pos;
        self.visited.insert((self.direction, new_pos));
    }

    // fn steps(&self) -> usize {
    //     self.visited.len()
    // }

    fn visited(&self) -> usize {
        let mut count = HashSet::new();

        for pos in self.visited.iter() {
            count.insert(pos.1);
        }

        count.len()
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> (HashSet<(usize, usize)>, (usize, usize), (usize, usize)) {
    let mut obstacles = HashSet::new();
    let mut start = (0, 0);
    let mut max_row = 0;

    for (row, line) in input.lines().enumerate() {
        max_row += 1;
        for (col, ch) in line.trim().chars().enumerate() {
            match ch {
                '#' => {
                    obstacles.insert((col, row));
                },
                '^' => start = (col, row),
                _ => continue,
            }
        }
    }

    (obstacles, start, (input.lines().next().unwrap().len() - 1, max_row - 1))
}

#[aoc(day6, part1)]
pub fn solve_part1(
    (obstacles, start, bounds): &(
        HashSet<(usize, usize)>,
        (usize, usize),
        (usize, usize),
    ),
) -> usize {
    let mut guard = Guard::new(start.clone(), bounds.clone());

    loop {
        match guard.look(obstacles) {
            Ok(_) => guard.step(),
            Err(MapError::Obstacle) => guard.direction.right(),
            Err(MapError::Bounds) => break,
        }
    }

    guard.visited()
}

// higher than 800,less than 2078. Also wrong: 1742, 1962, 1964, 1966, 1975, 2155. All of these give false positives in the test.
#[aoc(day6, part2)]
pub fn solve_part2(
    (obstacles, start, bounds): &(
        HashSet<(usize, usize)>,
        (usize, usize),
        (usize, usize),
    ),
) -> usize {
    let mut guard = Guard::new(start.clone(), bounds.clone());
    let mut obstructions = 0;
    // let mut count = 0;
    let mut objects = Vec::new();

    loop {
        match guard.look(obstacles) {
            Ok(new_pos) => {
                    let mut new_obs = obstacles.clone();
                    
                    new_obs.insert(new_pos);
                    if guard.test_loop(&new_obs) {
                        obstructions += 1;
                        objects.push(new_pos);
                    }
                    guard.step();
                    // if guard.glance(obstacles) {
                    //     obstructions += 1;
                    // }
                    // println!("Iteration: {}", count);
                },
            Err(MapError::Obstacle) => guard.direction.right(),
            Err(MapError::Bounds) => break,
        }
        // count += 1;
    }
    println!("{:?}", objects);

    obstructions
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 41);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 6);
    }
}
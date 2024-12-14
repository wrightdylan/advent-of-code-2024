use crate::prelude::*;

pub struct Robot {
    pos: Point,
    vel: Point,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            let pos: Vec<i32> = parts.0[2..].split(',').map(|num| num.parse::<i32>().unwrap()).collect();
            let vel: Vec<i32> = parts.1[2..].split(',').map(|num| num.parse::<i32>().unwrap()).collect();
            Robot { pos: Point::new(pos[0], pos[1]), vel: Point::new(vel[0], vel[1]) }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Vec<Robot>) -> usize {
    let mut pos = Vec::new();
    // Test boundary
    // let bounday: (i32, i32) = (11, 7);
    // Actual boundary
    let bounday: (i32, i32) = (101, 103);
    let steps = 100;

    for robot in input {
        let x = robot.pos.x + robot.vel.x * steps;
        let y = robot.pos.y + robot.vel.y * steps;
        let p = (
            ((x % bounday.0) + bounday.0) % bounday.0,
            ((y % bounday.1) + bounday.1) % bounday.1,
        );
        pos.push(p);
    }

    let mid_x = bounday.0 / 2;
    let mid_y = bounday.1 / 2; 
    
    // [tl, tr, bl, br]
    let mut quadrants: [usize; 4] = [0; 4];

    pos.iter().for_each(|p| {
        if p.0 != mid_x && p.1 != mid_y {
            let index = match (p.0 < mid_x, p.1 < mid_y) {
                (true, true) => 0,
                (true, false) => 2,
                (false, true) => 1,
                (false, false) => 3,
            };
            quadrants[index] += 1;
        }
    });

    quadrants.iter().product()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Vec<Robot>) -> usize {
    let boundary: (i32, i32) = (101, 103);
    let mut i = 0;
    'outer: loop {
        let mut pos = HashSet::new();

        for robot in input {
            let x = robot.pos.x + robot.vel.x * i as i32;
            let y = robot.pos.y + robot.vel.y * i as i32;
            let p = (
                ((x % boundary.0) + boundary.0) % boundary.0,
                ((y % boundary.1) + boundary.1) % boundary.1,
            );
            if !pos.contains(&p) {
                pos.insert(p);
            } else {
                break;
            }
        }

        // 4 hours to get to this point :...(
        // I tried all sorts of shit, including rendering every frame as an image to create an animation
        if pos.len() == input.len() {           
            // Uncomment if you want to see the tree
            // for y in 0..boundary.1 {
            //     for x in 0..boundary.0 {
            //         if pos.contains(&(x, y)) {
            //             print!("#");
            //         } else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }
            // println!("Iteration {}", i);
            break 'outer;
        }

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 12);
    }
}
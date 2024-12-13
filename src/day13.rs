#[derive(Debug)]
pub struct Crane {
    a: (isize, isize),
    b: (isize, isize),
    p: (isize, isize),
}

impl Crane {
    fn new() -> Self {
        Self {
            a: (0, 0),
            b: (0, 0),
            p: (0, 0),
        }
    }

    fn solve(&self, offset: isize) -> Option<(isize, isize)> {
        let pxo = self.p.0 + offset;
        let pyo = self.p.1 + offset;
        
        let denominator = self.b.1 * self.a.0 - self.b.0 * self.a.1;
        if denominator == 0 {
            return None;
        }
        
        let n = (pyo * self.a.0 - pxo * self.a.1) / denominator;
        
        let m = if self.a.0 != 0 {
            (pxo - n * self.b.0) / self.a.0
        } else if self.a.1 != 0 {
            (pyo - n * self.b.1) / self.a.1
        } else {
            return None;
        };
        
        if m >= 0 && n >= 0 &&
           m * self.a.0 + n * self.b.0 == pxo &&
           m * self.a.1 + n * self.b.1 == pyo {
            Some((m, n))
        } else {
            None
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Crane> {
    input
        .split("\n\n")
        .map(|block| {
            let mut crane = Crane::new();
            for line in block.lines() {
                let (name, pos) = line.split_once(": ").unwrap();
                let coords = pos.split_once(", ").unwrap();
                let x = coords.0[2..].parse().unwrap();
                let y = coords.1[2..].parse().unwrap();

                match name {
                    "Button A" => crane.a = (x, y),
                    "Button B" => crane.b = (x, y),
                    "Prize"    => crane.p = (x, y),
                    _ => unreachable!(),
                }
            }
            crane
        }).collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Vec<Crane>) -> isize {
    input
        .iter()
        .map(|crane| {
            if let Some((m, n)) = crane.solve(0) {
                m * 3 + n
            } else {
                0
            }
        }).sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Vec<Crane>) -> isize {
    input
        .iter()
        .map(|crane| {
            if let Some((m, n)) = crane.solve(10_000_000_000_000) {
                m * 3 + n
            } else {
                0
            }
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 480);
    }
}
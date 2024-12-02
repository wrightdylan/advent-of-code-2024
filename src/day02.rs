enum State {
    Up,
    Down,
    Init,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: &Vec<isize>) -> bool {
    let mut prev_diff = report[1] - report[0];

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 || (diff.signum() != prev_diff.signum()) {
            return false;
        }
        prev_diff = diff;
    }

    true
}

fn is_safe_stateful(report: &Vec<isize>) -> bool {
    let mut dir = State::Init;

    for window in report.windows(2) {
        let (prev, level) = (window[0], window[1]);

        match (dir, level - prev) {
            (State::Init, diff) if diff.abs() > 3 || diff.abs() == 0 => return false,
            (State::Init, diff) if diff > 0 => dir = State::Up,
            (State::Init, _) => dir = State::Down,

            (State::Up, diff) if diff > 3 || diff < 1 => return false,
            (State::Up, _) => dir = State::Up,

            (State::Down, diff) if diff < -3 || diff > -1 => return false,
            (State::Down, _) => dir = State::Down,
        }
    }

    true
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Vec<isize>>) -> usize {
    input
        .iter()
        .filter(|report| is_safe(report))
        .count()
}

#[aoc(day2, part1, Stateful)]
pub fn solve_part1_stateful(input: &Vec<Vec<isize>>) -> usize {
    input
        .iter()
        .filter(|report| is_safe_stateful(report))
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Vec<isize>>) -> usize {
    let mut safe = 0;

    for report in input.iter() {
        if is_safe(report) {
            safe += 1;
        } else {
            for idx in 0..report.len() {
                let mut sample = report.clone();
                sample.remove(idx);
                if is_safe(&sample) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    safe

    // This refactored code is marginally slower
    // input
    //     .iter()
    //     .filter(|report| {
    //         if is_safe(report) {
    //             return true;
    //         }
    //         report.iter().enumerate().any(|(idx, _)| {
    //             let sample: Vec<_> = report[..idx]
    //                 .iter()
    //                 .chain(report[idx + 1..].iter())
    //                 .copied()
    //                 .collect();
    //             is_safe(&sample)
    //         })
    //     }).count()
}

#[aoc(day2, part2, Stateful)]
pub fn solve_part2_stateful(input: &Vec<Vec<isize>>) -> usize {
    let mut safe = 0;

    for report in input.iter() {
        if is_safe_stateful(report) {
            safe += 1;
        } else {
            for idx in 0..report.len() {
                let mut sample = report.clone();
                sample.remove(idx);
                if is_safe_stateful(&sample) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "7 6 4 2 1
                        1 2 7 8 9
                        9 7 6 2 1
                        1 3 2 4 5
                        8 6 4 4 1
                        1 3 6 7 9";
    
    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 2);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 4);
    }
}
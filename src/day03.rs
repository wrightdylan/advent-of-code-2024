use regex::Regex;

#[derive(Debug)]
pub enum Instruction {
    Do,
    Dont,
    Mul(usize, usize),
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut inst: Vec<Instruction> = Vec::new();

    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do()"    => inst.push(Instruction::Do),
            "don't()" => inst.push(Instruction::Dont),
            _ => inst.push(Instruction::Mul(cap[2].parse().unwrap(), cap[3].parse().unwrap())),
        }
    }

    inst
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> usize {
    let mut acc = 0;
    
    for inst in input {
        match inst {
            Instruction::Mul(num1, num2) => acc += num1 * num2,
            _ => continue,
        }
    }

    acc
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> usize {
    let mut enabled = true;
    let mut acc = 0;

    for inst in input {
        match inst {
            Instruction::Do   => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(num1, num2) => {
                    if enabled {
                        acc += num1 * num2;
                    }
                },
        }
    }
    
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(&TEST1)), 161);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(&TEST2)), 48);
    }
}
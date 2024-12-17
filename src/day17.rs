use crate::prelude::*;

struct Machine {
    al: usize,
    bl: usize,
    cl: usize,
    ip: usize,
    cs: Vec<usize>,
    ss: Vec<usize>,
    os: bool,
}

impl Machine {
    // *** All the basic shit ***
    fn new(regs: &Vec<usize>, prog: &Vec<usize>) -> Self {
        Self { 
            al: regs[0], bl: regs[1], cl: regs[2],
            ip: 0,
            cs: prog.clone(),
            ss: Vec::new(),
            os: true,
        }
    }

    // Run the damned machine
    fn run(&mut self) {
        while self.os {
            match self.get_opcode() {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => unreachable!(),
            }
        }
    }

    // Get the opcode
    fn get_opcode(&self) -> usize {
        self.cs[self.ip]
    }

    // Retrieves the operand
    fn get_op(&self) -> usize {
        self.cs[self.ip + 1]
    }

    // Increment the instruction pointer
    fn inc_ptr(&mut self) {
        if self.ip < self.cs.len() - 2 {
            self.ip += 2;
        } else {
            self.hcf();
        }
    }

    // Used only for combo operands
    fn combo(&self, operand: usize) -> usize {
        match operand {
            4 => self.al,
            5 => self.bl,
            6 => self.cl,
            7 => unreachable!(),
            _ => operand,
        }
    }

    // Dump the contents of the Machine for study
    #[allow(dead_code)]
    fn core_dump(&self) {
        println!("al: {}", self.al);
        println!("bl: {}", self.bl);
        println!("cl: {}", self.cl);
        println!("ip: {}", self.ip);
        println!("cs: {:?}", self.cs);
        println!("ss: {:?}", self.ss);
        println!("os: {}", self.os);
    }

    // Get the final solution
    fn ss_to_string(&self) -> String {
        self.ss.iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    // *** All the opcode shit ***
    // Opcode 0 - divide al by combo operand
    fn adv(&mut self,) {
        self.al = self.al / 2_usize.pow(self.combo(self.get_op()) as u32);
        self.inc_ptr();
    }

    // Opcode 1 - bitwise XOR of bl and literal operand
    fn bxl(&mut self) {
        self.bl ^= self.get_op();
        self.inc_ptr();
    }

    // Opcode 2 - truncates the combo operand
    fn bst(&mut self) {
        self.bl = self.combo(self.get_op()) % 8;
        self.inc_ptr();
    }

    // Opcode 3 - jump not zero; places pointer to literal operand if al not zero
    fn jnz(&mut self) {
        if self.al != 0 {
            self.ip = self.get_op();
        } else {
            self.hcf();
        }
    }

    // Opcode 4 - bitwise XOR of bl and cl
    fn bxc(&mut self) {
        self.bl ^= self.cl;
        self.inc_ptr();
    }

    // Opcode 5 - outputs the combo operand after modulo
    fn out(&mut self) {
        self.ss.push(self.combo(self.get_op()) % 8);
        self.inc_ptr();
    }

    // Opcode 6 - divide al by combo operand but place in bl
    fn bdv(&mut self,) {
        self.bl = self.al / 2_usize.pow(self.combo(self.get_op()) as u32);
        self.inc_ptr();
    }

    // Opcode 7 - divide al by combo operand but place in cl
    fn cdv(&mut self,) {
        self.cl = self.al / 2_usize.pow(self.combo(self.get_op()) as u32);
        self.inc_ptr();
    }

    // Opcode X - stop everything
    fn hcf(&mut self) {
        self.os = false;
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ss.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(","))
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let parts = input.split_once("\n\n").unwrap();

    (
        parts.0
            .lines()
            .map(|line| {
                let(_, num) = line.split_once(": ").unwrap();
                num.parse().unwrap()
            }).collect(),
        parts.1
            .split_once(' ')
            .unwrap()
            .1
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect()
    )
}

#[aoc(day17, part1)]
pub fn solve_part1((regs, prog): &(Vec<usize>, Vec<usize>)) -> String {
    let mut cpu = Machine::new(regs, prog);
    // For testing internals
    // let mut cpu = Machine::new(&vec![0,2024,43690], &vec![4,0]);

    cpu.run();

    // cpu.core_dump();
    cpu.ss_to_string()
}

// This will be the real one
// #[aoc(day17, part2)]
// pub fn solve_part2((_, prog): &(Vec<usize>, Vec<usize>)) -> usize {
//     0
// }

// Good for this winter
#[aoc(day17, part2, BruteForce)]
pub fn solve_part2((_, prog): &(Vec<usize>, Vec<usize>)) -> usize {
    let mut i = 0;

    'outer: loop {
        let mut cpu = Machine::new(&vec![i, 0, 0], prog);
        print!("\rIteration: {} ",i);

        while cpu.os {
            match cpu.get_opcode() {
                0 => cpu.adv(),
                1 => cpu.bxl(),
                2 => cpu.bst(),
                3 => cpu.jnz(),
                4 => cpu.bxc(),
                5 => cpu.out(),
                6 => cpu.bdv(),
                7 => cpu.cdv(),
                _ => unreachable!(),
            }

            if cpu.ss.len() == cpu.cs.len() {
                if cpu.ss == cpu.cs {
                    break 'outer;
                } else {
                    break;
                }
            }
        }

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST2: &str = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";

    const TEST3: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST4: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), "0,1,2");
    }

    #[test]
    fn part1_test3() {
        assert_eq!(solve_part1(&input_generator(TEST3)), "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST4)), 117440);
    }
}
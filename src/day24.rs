use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Operator {
    And,
    Or,
    Xor,
}

// output is also the id of the gate
// State: unset = 0b00;
// State: set, false = 0b01;
// State: set, true = 0b11;
// Two stateful representations (for benchmarking purposes): 1) using the BitVec
// library, and 2) using a separate byte for each I/O point
// The BitVec uses a single byte with the mapping: 0b00xxyyzz,
// where xx is input 0, yy is input 1, and zz is the output.
// The Vec<u8> version has input x on byte 0, input y on byte 1, output z onbyte 
// Part 1 Standard runs consistently faster than the BitVector method
#[derive(Debug, Clone)]
pub struct Gate {
    output:   String,
    inputs:   (String, String),
    operator: Operator,
    terminal: bool,
    z_num:    usize,
    state:    BitVec,
    states:   Vec<u8>,
}

impl Gate {
    fn new(parts: &Vec<&str>) -> Self {
        let operator = match parts[1] {
            "AND" => Operator::And,
            "OR"  => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("Unknown operation")
        };

        let mut terminal = false;
        let mut z_num = 0;
        let (first, rest) = parts[4].split_at(1);
        if first == "z" {
            terminal = true;
            z_num = rest.parse::<usize>().unwrap();
        }

        Self {
            output: parts[4].to_string(),
            inputs: (parts[0].to_string(), parts[2].to_string()),
            operator,
            terminal,
            z_num,
            state: BitVec::with_capacity(6),
            states: vec![0, 0, 0],
        }
    }

    fn index_input(&self, input: &String) -> usize {
        if self.inputs.0 == *input {
            0
        } else {
            1
        }
    }

    fn bv_push_result(&self, result: &mut u64) {
        if self.state.get_bit(1) {
            *result = *result | 1 << self.z_num;
        }
    }

    fn push_result(&self, result: &mut u64) {
        if self.states[2] >> 1 & 1 == 1 {
            *result = *result | 1 << self.z_num;
        }
    }

    fn bv_both_in_set(&self) -> bool {
        self.state.get_bit(2) & self.state.get_bit(4)
    }

    fn both_in_set(&self) -> bool {
        self.states[0] & self.states[1] & 1 == 1
    }

    fn bv_get_out_state(&self) -> bool {
        self.state.get_bit(0)
    }

    fn get_out_state(&self) -> bool {
        self.states[2] & 1 == 1
    }

    fn bv_set_in_state(&mut self, input: &String, state: &bool) {
        let in_idx = self.index_input(input);
        match in_idx {
            0 => {
                self.state.set_bit(4, true);
                if *state == true {
                    self.state.set_bit(5, *state);
                }
            },
            1 => {
                self.state.set_bit(2, true);
                if *state == true {
                    self.state.set_bit(3, *state);
                }
            },
            _ => unreachable!()
        }
    }

    fn set_in_state(&mut self, input: &String, state: &bool) {
        let in_idx = self.index_input(input);
        match in_idx {
            0 => {
                self.states[0] = 1;
                if *state == true {
                    self.states[0] |= (*state as u8) << 1;
                }
            },
            1 => {
                self.states[1] = 1;
                if *state == true {
                    self.states[1] |= (*state as u8) << 1;
                }
            },
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Circuit {
    gates:  HashMap<String, Gate>,
    inputs: HashMap<String, Vec<String>>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            gates:  HashMap::new(),
            inputs: HashMap::new()
        }
    }

    fn add_gate(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        self.inputs.entry(parts[0].to_string())
            .or_insert(Vec::new())
            .push(parts[4].to_string());
        self.inputs.entry(parts[2].to_string())
            .or_insert(Vec::new())
            .push(parts[4].to_string());
        let gate = Gate::new(&parts);
        self.gates.insert(parts[4].to_string(), gate);
    }

    fn get_gate_by_id(&mut self, gate_id: &String) -> &mut Gate {
        self.gates.get_mut(gate_id).unwrap()
    }
}

fn to_bool(&state: &usize) -> bool {
    state != 0
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> (Vec<(String, usize)>, Circuit) {
    let mut circuit = Circuit::new();

    let (wires, gates) = input.split_once("\n\n").unwrap();
    let inputs: Vec<(String, usize)> = wires
        .lines()
        .map(|line| {
            let (in_pin, val) = line.split_once(": ").unwrap();
            (in_pin.to_string(), val.parse::<usize>().unwrap())
        })
        .collect();

    for line in gates.lines() {
        circuit.add_gate(line);
    }
    
    (inputs, circuit)
}

#[aoc(day24, part1, BitVector)]
pub fn solve_part1_bitvector((init, origin_circuit): &(Vec<(String, usize)>, Circuit)) -> u64 {
    let mut result: u64 = 0;
    let mut circuit = origin_circuit.clone();
    let mut inputs = VecDeque::new();
    inputs.extend(init.clone());

    while let Some((input, state)) = inputs.pop_front() {
        let gate_ids = circuit.inputs.get(&input).cloned().unwrap();
        for gate_id in gate_ids {
            let gate = circuit.get_gate_by_id(&gate_id);
            if !gate.bv_get_out_state() {
                let bool_state = to_bool(&state);
                match gate.operator {
                    Operator::And => {
                        if !bool_state {
                            gate.state.set_bit(0, true);
                            if gate.terminal {
                                gate.bv_push_result(&mut result);
                            } else {
                                inputs.push_back((gate.output.clone(), 0).clone());
                            }
                            continue;
                        } else {
                            gate.bv_set_in_state(&input, &bool_state);
                        }
                    },
                    Operator::Or  => {
                        if bool_state {
                            gate.state.set_bit(0, true);
                            gate.state.set_bit(1, true);
                            if gate.terminal {
                                gate.bv_push_result(&mut result);
                            } else {
                                inputs.push_back((gate.output.clone(), 1).clone());
                            }
                            continue;
                        } else {
                            gate.bv_set_in_state(&input, &bool_state);
                        }
                    },
                    Operator::Xor => gate.bv_set_in_state(&input, &bool_state),
                }
                if gate.bv_both_in_set() {
                    match gate.operator {
                        Operator::And => {
                            let out_state = gate.state.get_bit(3) & gate.state.get_bit(5);
                            gate.state.set_bit(0, true);
                            gate.state.set_bit(1, out_state);
                            if gate.terminal {
                                gate.bv_push_result(&mut result);
                            } else {
                                inputs.push_back((gate.output.clone(), out_state as usize));
                            }
                        },
                        Operator::Or  => {
                            gate.state.set_bit(0, true);
                            if !gate.terminal {
                                inputs.push_back((gate.output.clone(), 0));
                            }
                        },
                        Operator::Xor => {
                            let out_state = gate.state.get_bit(3) ^ gate.state.get_bit(5);
                            gate.state.set_bit(0, true);
                            gate.state.set_bit(1, out_state);
                            if gate.terminal {
                                gate.bv_push_result(&mut result);
                            } else {
                                inputs.push_back((gate.output.clone(), out_state as usize));
                            }
                        },
                    }
                }
            }
        }
    }

    result
}

#[aoc(day24, part1, Standard)]
pub fn solve_part1_standard((init, origin_circuit): &(Vec<(String, usize)>, Circuit)) -> u64 {
    let mut result: u64 = 0;
    let mut circuit = origin_circuit.clone();
    let mut inputs = VecDeque::new();
    inputs.extend(init.clone());

    while let Some((input, state)) = inputs.pop_front() {
        let gate_ids = circuit.inputs.get(&input).cloned().unwrap();
        for gate_id in gate_ids {
            let gate = circuit.get_gate_by_id(&gate_id);
            if !gate.get_out_state() {
                let bool_state = to_bool(&state);
                match gate.operator {
                    Operator::And => {
                        if !bool_state {
                            gate.states[2] = 1;
                            if gate.terminal {
                                gate.push_result(&mut result);
                            } else {
                                inputs.push_back((gate.output.clone(), 0).clone());
                            }
                            continue;
                        } else {
                            gate.set_in_state(&input, &bool_state);
                        }
                    },
                    Operator::Or  => {
                        if bool_state {
                            gate.states[2] = 3;
                            if gate.terminal {
                                gate.push_result(&mut result);
                            } else {
                                inputs.push_back((gate.output.clone(), 1).clone());
                            }
                            continue;
                        } else {
                            gate.set_in_state(&input, &bool_state);
                        }
                    },
                    Operator::Xor => gate.set_in_state(&input, &bool_state),
                }
                if gate.both_in_set() {
                    match gate.operator {
                        Operator::And => {
                            let out_state = (gate.states[0] & gate.states[1]) >> 1;
                            gate.states[2] = 1;
                            gate.states[2] |= out_state << 1;
                            if gate.terminal {
                                gate.push_result(&mut result);
                            } else {
                                inputs.push_back((gate.output.clone(), out_state as usize));
                            }
                        },
                        Operator::Or  => {
                            gate.state.set_bit(0, true);
                            if !gate.terminal {
                                inputs.push_back((gate.output.clone(), 0));
                            }
                        },
                        Operator::Xor => {
                            let out_state = (gate.states[0] ^ gate.states[1]) >> 1;
                            gate.states[2] = 1;
                            gate.states[2] |= out_state << 1;
                            if gate.terminal {
                                gate.push_result(&mut result);
                            } else {
                                inputs.push_back((gate.output.clone(), out_state as usize));
                            }
                        },
                    }
                }
            }
        }
    }

    result
}

#[aoc(day24, part2)]
pub fn solve_part2((init, origin_circuit): &(Vec<(String, usize)>, Circuit)) -> String {
    let mut result_x: u64 = 0;
    let mut result_y: u64 = 0;

    for (item, state) in init {
        let (input, rest) = item.split_at(1);
        let offset = rest.parse::<u64>().unwrap();
        match input {
            "x" => result_x |= (*state as u64) << offset,
            "y" => result_y |= (*state as u64) << offset,
            _   => unreachable!()
        }
    }
    println!("X: {}, Y: {}, Z: {}", result_x, result_y, result_x + result_y);
    "Nothing".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const TEST2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    const TEST3: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

    #[test]
    fn part1_test1_bitvector() {
        assert_eq!(solve_part1_bitvector(&input_generator(TEST1)), 4);
    }

    #[test]
    fn part1_test1_standard() {
        assert_eq!(solve_part1_standard(&input_generator(TEST1)), 4);
    }

    #[test]
    fn part1_test2_bitvector() {
        assert_eq!(solve_part1_bitvector(&input_generator(TEST2)), 2024);
    }

    #[test]
    fn part1_test2_standard() {
        assert_eq!(solve_part1_standard(&input_generator(TEST2)), 2024);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST3)), "z00,z01,z02,z05");
    }
}
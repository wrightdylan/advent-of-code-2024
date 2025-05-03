#![allow(dead_code)]
use std::mem::swap;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn is_and(&self) -> bool {
        *self == Operator::And
    }

    fn is_or(&self) -> bool {
        *self == Operator::Or
    }

    fn is_xor(&self) -> bool {
        *self == Operator::Xor
    }
}

// output is also the id of the gate
// State: unset = 0b00;
// State: set, false = 0b01;
// State: set, true = 0b11;
// Two stateful representations (for benchmarking purposes): 1) using the BitVec
// library, and 2) using a separate byte for each I/O point
// The BitVec uses a single byte with the mapping: 0b00xxyyzz,
// where xx is input 0, yy is input 1, and zz is the output.
// The Vec<u8> version has input x on byte 0, input y on byte 1, output z on byte 2
// UPDATE: states is in the order of [a, b, q]
// Part 1 Standard runs consistently faster than the BitVector method
#[derive(Debug, Clone)]
pub struct Gate {
    q:        String,
    a:        String,
    b:        String,
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
        if let Some((first, rest)) = parse_gate_io(parts[4]) {
            if first == "z" {
                terminal = true;
                z_num = rest;
            }
        }

        // Sorting inputs alphabetically may make part 2 a bit easier
        let (a, b) = {
            let mut inputs = vec![parts[0], parts[2]];
            inputs.sort();

            (inputs[0].to_string(), inputs[1].to_string())
        };

        Self {
            q: parts[4].to_string(),
            a,
            b,
            operator,
            terminal,
            z_num,
            state: BitVec::with_capacity(6),
            states: vec![0, 0, 0],
        }
    }

    fn index_input(&self, input: &String) -> usize {
        if self.a == *input {
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
        let idx = self.index_input(input);
        self.states[idx] = 1 | ((*state as u8) << 1);
    }

    fn is_xy_input(&self) -> bool {
        if parse_gate_io(&self.a).is_some() {
            if parse_gate_io(&self.b).is_some() {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct Circuit {
    gates:  HashMap<String, Gate>,
    inputs: HashMap<String, Vec<String>>,
    width:  usize, // bit width of output
}

impl Circuit {
    fn new() -> Self {
        Self {
            gates:  HashMap::new(),
            inputs: HashMap::new(),
            width: 0,
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
        if gate.terminal {
            self.width += 1;
        }
        self.gates.insert(parts[4].to_string(), gate);
    }

    fn get_gate_by_id(&mut self, gate_id: &String) -> &mut Gate {
        self.gates.get_mut(gate_id).unwrap()
    }

    // Inspects the structure of each adder. First output should just be XOR,
    // second and last outputs should be half-adders, and every other output
    // should be a full-adder. The complete structure is a ripple-carry adder.
    // The adder for the current output should be XOR, and the carry-adder
    // from the previous output should be an AND.
    fn inspect_adder(&self) -> Vec<String> {
        let width_digits = self.width.to_string().len();
        let mut swaps: Vec<String> = Vec::new();

        for i in 0..self.width {
            let gate_name = format!("z{:0width$}", i, width = width_digits);
            let gate = self.gates.get(&gate_name).unwrap();

            let is_last = i == self.width - 1;
            if let Some(bad_q) = self._descend_adder(gate, 0, gate.z_num, is_last) {
                swaps.push(bad_q);
            }
        }

        swaps
    }

    fn _descend_adder(&self, gate: &Gate, depth: usize, z_num: usize, last: bool) -> Option<String> {
        let q = gate.q.clone();
        if depth == 0 && gate.operator.is_and() {
            return Some(q)
        } else if gate.is_xy_input() {
            return None;
        }
        let gate_a = self.gates.get(&gate.a).unwrap();
        let gate_b = self.gates.get(&gate.b).unwrap();

        match depth {
            0 => {
                if (last && !gate.operator.is_or()) || (!last && !gate.operator.is_xor()) {
                    return Some(q);
                }
            },
            1 => {
                if last {
                    return if gate.operator.is_and() { None } else { Some(q) }
                } else if !gate.operator.is_or() {
                    return Some(q);
                }
            },
            2 => {
                if !gate.operator.is_and() {
                    return Some(q);
                }
            },
            _ => return None,
        }

        self._descend_adder(gate_a, depth + 1, z_num, last)
            .or_else(|| self._descend_adder(gate_b, depth + 1, z_num, last))
    }

    pub fn swap_outputs(&mut self, q1: &str, q2: &str) {
        // Clone the gates to work with
        let mut gate1 = self.gates.remove(q1).unwrap();
        let mut gate2 = self.gates.remove(q2).unwrap();
        
        // Update input mappings - remove old mappings first
        self.remove_input_mappings(q1, &gate1.a, &gate1.b);
        self.remove_input_mappings(q2, &gate2.a, &gate2.b);
        
        // Swap required values
        swap(&mut gate1.q, &mut gate2.q);
        swap(&mut gate1.terminal, &mut gate2.terminal);
        swap(&mut gate1.z_num, &mut gate2.z_num);
        
        // Add back the gates with swapped q values
        self.gates.insert(gate1.q.clone(), gate1.clone());
        self.gates.insert(gate2.q.clone(), gate2.clone());
        
        // Re-add input mappings with new q values
        self.add_input_mappings(&gate1.q, &gate1.a, &gate1.b);
        self.add_input_mappings(&gate2.q, &gate2.a, &gate2.b);
    }
    
    fn remove_input_mappings(&mut self, gate_q: &str, input_a: &str, input_b: &str) {
        // Remove gate from input_a's list
        if let Some(outputs) = self.inputs.get_mut(input_a) {
            outputs.retain(|q| q != gate_q);
            if outputs.is_empty() {
                self.inputs.remove(input_a);
            }
        }
        
        // Remove gate from input_b's list
        if let Some(outputs) = self.inputs.get_mut(input_b) {
            outputs.retain(|q| q != gate_q);
            if outputs.is_empty() {
                self.inputs.remove(input_b);
            }
        }
    }
    
    fn add_input_mappings(&mut self, gate_q: &str, input_a: &str, input_b: &str) {
        // Add gate to input_a's list
        self.inputs
            .entry(input_a.to_string())
            .or_insert_with(Vec::new)
            .push(gate_q.to_string());
        
        // Add gate to input_b's list
        self.inputs
            .entry(input_b.to_string())
            .or_insert_with(Vec::new)
            .push(gate_q.to_string());
    }
}

fn to_bool(&state: &usize) -> bool {
    state != 0
}

fn get_bit_values(u64_number: u64) -> String {
    let mut bit_values = String::new();
    for i in (0..64).rev() {
        let bit = (u64_number >> i) & 1;
        bit_values.push_str(&bit.to_string());
    }
    bit_values
}

fn parse_gate_io(term: &str) -> Option<(&str, usize)> {
    let (first, rest) = term.split_at(1);
    match first {
        "x" | "y" | "z" => Some((first, rest.parse::<usize>().unwrap())),
        _ => None
    }
}

fn print_64bit_index(spacing: usize, label: Option<&str>) {
    let line: String = std::iter::repeat('-').take(66 + spacing).collect();
    println!("{line}");

    let mut first_row = String::new();
    let mut second_row = String::new();

    match label {
        Some(text) => {
            let truncated_label = &text[..spacing.min(text.len())];
            let spaces = " ".repeat(spacing.saturating_sub(truncated_label.len()));
            first_row.push_str(truncated_label);
            first_row.push_str(&spaces);
        },
        None => first_row.push_str(&" ".repeat(spacing))
    }
    first_row.push_str(": ");

    second_row.push_str(&" ".repeat(spacing));
    second_row.push_str(": ");

    for num in (0..64).rev() {
        let num_str = num.to_string();

        match num_str.len() {
            1 => {
                first_row.push_str(&num_str);
            },
            _ => {
                first_row.push_str(&num_str[..1]);
                second_row.push_str(&num_str[1..]);
            }
        }
    }

    println!("{first_row}");
    println!("{second_row}");
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
                                inputs.push_back((gate.q.clone(), 0).clone());
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
                                inputs.push_back((gate.q.clone(), 1).clone());
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
                                inputs.push_back((gate.q.clone(), out_state as usize));
                            }
                        },
                        Operator::Or  => {
                            gate.state.set_bit(0, true);
                            if !gate.terminal {
                                inputs.push_back((gate.q.clone(), 0));
                            }
                        },
                        Operator::Xor => {
                            let out_state = gate.state.get_bit(3) ^ gate.state.get_bit(5);
                            gate.state.set_bit(0, true);
                            gate.state.set_bit(1, out_state);
                            if gate.terminal {
                                gate.bv_push_result(&mut result);
                            } else {
                                inputs.push_back((gate.q.clone(), out_state as usize));
                            }
                        },
                    }
                }
            }
        }
    }

    result
}

// A slight refactor for simplification. It turns out that 'shortcuts' don't really improve performance.
#[aoc(day24, part1, Standard)]
pub fn solve_part1_standard((init, origin_circuit): &(Vec<(String, usize)>, Circuit)) -> u64 {
    let mut result: u64 = 0;
    let mut circuit = origin_circuit.clone();
    let mut inputs = VecDeque::new();
    inputs.extend(init.clone());

    // Hijacking part 1 to manually solve part 2
    // circuit.swap_outputs("hdt", "z05");
    // circuit.swap_outputs("gbf", "z09");
    // circuit.swap_outputs("jgt", "mht");
    // circuit.swap_outputs("nbf", "z30");

    while let Some((input, state)) = inputs.pop_front() {
        let bool_state = state == 1;
        if let Some(gate_ids) = circuit.inputs.get(&input).cloned() {
            for gate_id in gate_ids {
                let gate = circuit.get_gate_by_id(&gate_id);
                if !gate.get_out_state() {
                    gate.set_in_state(&input, &bool_state);

                    if gate.both_in_set() {
                        let out_state = match gate.operator {
                            Operator::And => (gate.states[0] & gate.states[1]) >> 1,
                            Operator::Or  => (gate.states[0] | gate.states[1]) >> 1,
                            Operator::Xor => (gate.states[0] ^ gate.states[1]) >> 1,
                        };

                        gate.states[2] = 1 | (out_state << 1);
                    
                        if gate.terminal {
                            gate.push_result(&mut result);
                        } else {
                            inputs.push_back((gate.q.clone(), out_state as usize));
                        }
                    }
                }
            }
        }
    }

    result
}

#[aoc(day24, part2)]
pub fn solve_part2((_, circuit): &(Vec<(String, usize)>, Circuit)) -> String {
    // let mut result_x: u64 = 0;
    // let mut result_y: u64 = 0;
    // let circuit = origin_circuit.clone();

    // Manual finder
    // Result from part 1
    // let wrong = 51657025112326;
    // let wrong = 51657025112294;
    // let wrong = 51657025111782;
    // let wrong = 51657025079014;
    // let wrong = 51658098820838; // This is correct

    // for (item, state) in init {
    //     let (input, offset_str) = item.split_at(1);
    //     let offset = offset_str.parse::<u64>().unwrap();
    //     match input {
    //         "x" => result_x |= (*state as u64) << offset,
    //         "y" => result_y |= (*state as u64) << offset,
    //         _   => unreachable!()
    //     }
    // }
    // let result_z= result_x + result_y;
    // println!("X: {}, Y: {}, Z: {}", result_x, result_y, result_z);

    // These are the positions where the bit is flipped
    // let differences = result_z ^ wrong;
    // println!("Difference   : {}", result_z - wrong);
    // println!("Correct value: {}", get_bit_values(result_z));
    // println!("Wrong value  : {}", get_bit_values(wrong));
    // println!("Differences  : {}", get_bit_values(differences));
    // print_64bit_index(13, Some("Index"));

    let mut swaps = circuit.inspect_adder();
    swaps.sort();
    
    swaps.join(",")
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
}
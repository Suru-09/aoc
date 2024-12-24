use std::collections::{HashMap, VecDeque};

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let input = utils::read_file(INPUT);
    let (wires, gates) = parse(&input);
    let mut wires_map: HashMap<&str, u8> = wires.into_iter().collect();
    compute_values(&mut wires_map, &gates);
    let mut wires_vec = wires_map
        .iter()
        .filter(|(wire, _)| wire.contains("z"))
        .map(|(key, val)| (*key, *val))
        .collect::<Vec<(&str, u8)>>();
    wires_vec.sort_by(|a, b| b.cmp(a)); // sort reversed
    println!("Wires: {:?}", wires_vec);

    let result = wires_vec
        .iter()
        .fold(0u128, |acc, (_, val)| (acc | *val as u128) << 1);
    println!("Result for part1: {}", result >> 1);
}

pub fn solve_part_2() {}

fn compute_values<'a>(wires: &mut HashMap<&'a str, u8>, gates: &'a Vec<Gate>) {
    let mut queue = gates.iter().collect::<VecDeque<_>>();
    while let Some(gate) = queue.pop_front() {
        let opt1 = wires.get(&gate.input1);
        let opt2 = wires.get(&gate.input2);

        if opt1.is_none() || opt2.is_none() {
            queue.push_back(gate);
            continue;
        }

        let input1 = *opt1.unwrap();
        let input2 = *opt2.unwrap();

        let output = match gate.gate_type {
            GateType::AND => input1 & input2,
            GateType::OR => input1 | input2,
            GateType::XOR => input1 ^ input2,
        };

        wires.insert(gate.output, output);
    }
}

#[derive(Debug)]
enum GateType {
    XOR,
    OR,
    AND,
}

#[derive(Debug)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    output: &'a str,
    gate_type: GateType,
}

impl<'a> Gate<'a> {
    pub fn new(input1: &'a str, input2: &'a str, output: &'a str, gate_type: GateType) -> Self {
        Self {
            input1,
            input2,
            output,
            gate_type,
        }
    }
}

fn parse_wire(wire: &str) -> (&str, u8) {
    let name_val = wire.trim().split(": ").collect::<Vec<&str>>();
    assert_eq!(name_val.len(), 2);
    (name_val[0], name_val[1].trim().parse::<u8>().unwrap())
}

fn parse_gate(gate: &str) -> Gate {
    let inputs_output = gate.trim().split(" -> ").collect::<Vec<&str>>();
    assert_eq!(inputs_output.len(), 2);
    let output = inputs_output[1];
    let input1 = &inputs_output[0][0..3];
    let input2;
    let mut gate_type = GateType::XOR;
    if inputs_output[0].contains("XOR") || inputs_output[0].contains("AND") {
        input2 = &inputs_output[0][8..11];
        if inputs_output[0].contains("AND") {
            gate_type = GateType::AND;
        }
    } else {
        input2 = &inputs_output[0][7..10];
        gate_type = GateType::OR;
    }
    Gate::new(input1, input2, output, gate_type)
}

fn parse(input: &str) -> (Vec<(&str, u8)>, Vec<Gate>) {
    let file = input.trim().split("\n\n").collect::<Vec<&str>>();
    assert_eq!(file.len(), 2);

    let wires = file[0]
        .lines()
        .map(|line| parse_wire(&line))
        .collect::<Vec<(&str, u8)>>();
    let gates = file[1]
        .lines()
        .map(|line| parse_gate(&line))
        .collect::<Vec<Gate>>();
    (wires, gates)
}

mod utils {
    pub fn read_file(path: &str) -> String {
        std::fs::read_to_string(path).unwrap()
    }

    pub fn read_file_lines(path: &str) -> Vec<String> {
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
    }
}

fn main() {
    solve_part_1();
    solve_part_2();
}

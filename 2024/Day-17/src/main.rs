const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let input_str = utils::read_file(INPUT);
    let mut computer = parse_computer(&input_str);
    computer.execute_instructions();

    let result = computer
        .result
        .iter()
        .fold(String::new(), |curr, nxt| curr + &nxt.to_string() + ",");
    println!("Result for part1: {:?}", result);
}

pub fn solve_part_2() {
    let input_str = utils::read_file(INPUT);
    let mut computer = parse_computer(&input_str);
    let instr_seq = computer.instr_sequence.clone();
    let target = instr_seq
        .iter()
        .map(|(opc, combo_op)| vec![opc.value(), combo_op.value()])
        .collect::<Vec<Vec<usize>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<usize>>();

    let rax_val = dfs_search(&mut computer, 0, target.len() as isize - 1, &target);

    println!("Result for part2: {:?}", rax_val);
}

fn dfs_search(computer: &mut Computer, reg_a: usize, depth: isize, target: &Vec<usize>) -> usize {
    if depth == -1 {
        return reg_a;
    }

    for i in 0..8 {
        computer.rax = reg_a * 8 + i;
        computer.rbx = 0;
        computer.rcx = 0;
        computer.result.clear();
        computer.execute_instructions();
        if computer.result[0] == target[depth as usize] {
            let result = dfs_search(computer, (reg_a * 8 + i), depth - 1, target);
            if result > 0 {
                return result;
            }
        }
    }

    0
}

fn parse_computer(input: &str) -> Computer {
    let register_programs = input.trim().split("\n\n").collect::<Vec<&str>>();
    assert_eq!(register_programs.len(), 2);

    let registers = register_programs[0];
    let registers_vec = registers.trim().split("\n").collect::<Vec<&str>>();
    assert_eq!(registers_vec.len(), 3);

    let rax = parse_register(registers_vec[0], 'A');
    let rbx = parse_register(registers_vec[1], 'B');
    let rcx = parse_register(registers_vec[2], 'C');

    let program = register_programs[1];
    let instr_sequence = parse_program(program);

    Computer::new(rax, rbx, rcx, instr_sequence)
}

fn parse_register(line: &str, register: char) -> usize {
    let formated = format!("Register {}: ", register);
    let pos = line.find(&formated).unwrap();
    let skip = formated.len();
    line[pos + skip..].trim().parse::<usize>().unwrap()
}

fn parse_program(line: &str) -> Vec<(OpCode, ComboOperand)> {
    let needle = "Program: ";
    let pos = line.find(needle).unwrap();
    let skip = needle.len();
    let values = line[pos + skip..].trim().split(",").collect::<Vec<&str>>();
    let chunks = values
        .chunks(2)
        .map(|c| c.to_vec().into_iter().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut instr_seq = vec![];
    for op in chunks {
        let op_code = OpCode::parse(op[0].trim().parse::<usize>().unwrap());
        let combo_op = ComboOperand::parse(op[1].trim().parse::<usize>().unwrap());
        instr_seq.push((op_code, combo_op));
    }
    instr_seq
}

#[derive(Debug, Clone, PartialEq)]
enum OpCode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl OpCode {
    pub fn value(&self) -> usize {
        match *self {
            OpCode::ADV => 0,
            OpCode::BXL => 1,
            OpCode::BST => 2,
            OpCode::JNZ => 3,
            OpCode::BXC => 4,
            OpCode::OUT => 5,
            OpCode::BDV => 6,
            OpCode::CDV => 7,
        }
    }

    pub fn parse(op_code: usize) -> OpCode {
        match op_code {
            0 => OpCode::ADV,
            1 => OpCode::BXL,
            2 => OpCode::BST,
            3 => OpCode::JNZ,
            4 => OpCode::BXC,
            5 => OpCode::OUT,
            6 => OpCode::BDV,
            7 => OpCode::CDV,
            _ => panic!("Invalid Op code!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ComboOperand {
    LiteralValue(usize),
    RAX,
    RBX,
    RCX,
}

impl ComboOperand {
    pub fn value(&self) -> usize {
        match *self {
            ComboOperand::LiteralValue(val) => val,
            ComboOperand::RAX => 4,
            ComboOperand::RBX => 5,
            ComboOperand::RCX => 6,
        }
    }

    pub fn parse(operand: usize) -> ComboOperand {
        match operand {
            4 => ComboOperand::RAX,
            5 => ComboOperand::RBX,
            6 => ComboOperand::RCX,
            0 | 1 | 2 | 3 | 7 => ComboOperand::LiteralValue(operand),
            _ => panic!("Invalid ComboOperand!"),
        }
    }
}

#[derive(Debug)]
struct Computer {
    pub rax: usize,
    pub rbx: usize,
    pub rcx: usize,
    pub instr_sequence: Vec<(OpCode, ComboOperand)>,
    pub result: Vec<usize>,
}

impl Computer {
    pub fn new(
        rax: usize,
        rbx: usize,
        rcx: usize,
        instructions: Vec<(OpCode, ComboOperand)>,
    ) -> Self {
        Self {
            rax,
            rbx,
            rcx,
            instr_sequence: instructions,
            result: vec![],
        }
    }

    fn execute_instr(&mut self, instr: (OpCode, ComboOperand), ip: &mut usize) {
        let (op_code, combo_op) = instr;
        let cop_val = match combo_op {
            ComboOperand::LiteralValue(val) => val,
            ComboOperand::RAX => self.rax,
            ComboOperand::RBX => self.rbx,
            ComboOperand::RCX => self.rcx,
            _ => panic!("Wrong combo op!"),
        };

        let result = match op_code {
            OpCode::ADV => {
                self.rax = self.rax / (2usize.pow(cop_val as u32));
                None
            }
            OpCode::BXL => {
                self.rbx = self.rbx ^ cop_val;
                None
            }
            OpCode::BST => {
                self.rbx = cop_val % 8;
                None
            }
            OpCode::JNZ => {
                if self.rax != 0 {
                    *ip = cop_val / 2;
                } else {
                    *ip += 1;
                }
                None
            }
            OpCode::BXC => {
                self.rbx = self.rbx ^ self.rcx;
                None
            }
            OpCode::OUT => Some(cop_val % 8),
            OpCode::BDV => {
                self.rbx = self.rax / (2usize.pow(cop_val as u32));
                None
            }
            OpCode::CDV => {
                self.rcx = self.rax / (2usize.pow(cop_val as u32));
                None
            }
            _ => todo!(),
        };

        if let Some(result) = result {
            self.result.push(result);
        }

        if op_code != OpCode::JNZ {
            *ip += 1;
        }
    }

    pub fn execute_instructions(&mut self) {
        let mut ip = 0;
        while ip < self.instr_sequence.len() {
            self.execute_instr(self.instr_sequence[ip].clone(), &mut ip);
        }
    }

    pub fn is_result_copy_program(&self) -> bool {
        //println!("Result: {:?}", self.result);
        //println!("Instruction sequence {:?}", self.instr_sequence);
        if self.result.len() != self.instr_sequence.len() * 2 {
            return false;
        }

        for i in (0..self.result.len() - 1).step_by(2) {
            if self.result[i] != self.instr_sequence[i / 2].0.value() {
                return false;
            }

            if self.result[i + 1] != self.instr_sequence[i / 2].1.value() {
                return false;
            }
        }

        true
    }
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

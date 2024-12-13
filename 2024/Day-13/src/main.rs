const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let claw_machines = parse_configurations(INPUT);
    //println!("Claw machines: {:?}", claw_machines);
    let tokens = claw_machines.iter().fold(0u128, |tokens, claw| {
        let least_tokens = find_least_tokens(&claw, false);
        //println!("Least tokenss: {}", least_tokens);
        tokens + least_tokens as u128
    });
    println!("Result for part1: {}", tokens);
}

pub fn solve_part_2() {
    let claw_machines = parse_configurations(INPUT);
    let tokens = claw_machines.iter().fold(0u128, |tokens, claw| {
        let least_tokens = find_least_tokens(&claw, true);
        //println!("Least tokenss: {}", least_tokens);
        tokens + least_tokens as u128
    });
    println!("Result for part2: {}", tokens);
}

fn find_least_tokens(claw_machine: &ClawMachine, is_part_2: bool) -> usize {
    let (px, py);

    if !is_part_2 {
        (px, py) = (claw_machine.prize.0 as isize, claw_machine.prize.1 as isize);
    } else {
        (px, py) = (
            claw_machine.prize.0 as isize + 10000000000000,
            claw_machine.prize.1 as isize + 10000000000000,
        );
    }

    let (ax, ay) = (
        claw_machine.button_a.0 as isize,
        claw_machine.button_a.1 as isize,
    );
    let (bx, by) = (
        claw_machine.button_b.0 as isize,
        claw_machine.button_b.1 as isize,
    );

    // the equation is the following:
    // times_a_pressed * ax + times_b_pressed * bx = px
    // times_a_pressed * ay + times_b_pressed * by = py
    // after a simple substituion:
    let times_b_pressed = (py * ax - px * ay) / (by * ax - bx * ay);
    let times_a_pressed = (px - times_b_pressed * bx) / ax;

    // println!(
    //     "A presses: {}, B presses: {}",
    //     times_a_pressed, times_b_pressed
    // );

    let result = 3 * times_a_pressed + times_b_pressed;
    if (times_a_pressed * ax + times_b_pressed * bx == px
        && times_a_pressed * ay + times_b_pressed * by == py)
    {
        result as usize
    } else {
        0
    }
}

#[derive(Debug)]
struct ClawMachine {
    pub button_a: (usize, usize),
    pub button_b: (usize, usize),
    pub prize: (usize, usize),
}

impl ClawMachine {
    pub fn new(a: (usize, usize), b: (usize, usize), p: (usize, usize)) -> Self {
        Self {
            button_a: a,
            button_b: b,
            prize: p,
        }
    }
}

fn extract_coords(line: &str, searched: &str) -> (usize, usize) {
    let coords = line[searched.len()..]
        .split(", ")
        .map(|str| str.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|str| str.trim()[2..].parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    assert_eq!(coords.len(), 2);
    if coords.len() == 2 {
        (coords[0], coords[1])
    } else {
        (0, 0)
    }
}

fn parse_configurations(input: &str) -> Vec<ClawMachine> {
    let mut configurations = vec![];
    let input_str = utils::read_file(input);
    let configs = input_str.trim().split("\n\n").collect::<Vec<&str>>();
    for config in configs {
        let claw_machine_parts = config.split("\n").collect::<Vec<&str>>();
        assert_eq!(claw_machine_parts.len(), 3);
        let (x1, y1) = extract_coords(claw_machine_parts[0], "Button A: ");
        let (x2, y2) = extract_coords(claw_machine_parts[1], "Button B: ");
        let (x3, y3) = extract_coords(claw_machine_parts[2], "Prize: ");
        configurations.push(ClawMachine::new((x1, y1), (x2, y2), (x3, y3)));
    }
    configurations
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
    solve_part_1(); // 36571
    solve_part_2(); // 85527711500010
}

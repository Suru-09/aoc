const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let lines = utils::read_file_lines(INPUT)
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let result = lines.iter().fold(0usize, |sum, line| {
        let (target, numbers) = parse_line(line);
        sum + calculate_calibration_res(target, &numbers, &vec!['+', '*'], 0usize, 0usize)
    });

    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let lines = utils::read_file_lines(INPUT)
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let result = lines.iter().fold(0usize, |sum, line| {
        let (target, numbers) = parse_line(line);
        sum + calculate_calibration_res(target, &numbers, &vec!['+', '*', '|'], 0usize, 0usize)
    });

    println!("Result for part2: {}", result);
}

fn calculate_calibration_res(
    target: usize,
    numbers: &Vec<usize>,
    allowed_op: &Vec<char>,
    depth: usize,
    acc: usize,
) -> usize {
    if depth == numbers.len() && acc == target {
        return target;
    } else if depth == numbers.len() {
        return 0;
    }

    for op in allowed_op.iter() {
        let new_acc = match op {
            '+' => acc + numbers[depth],
            '*' => acc * numbers[depth],
            '|' => (acc.to_string() + &numbers[depth].to_string())
                .parse::<usize>()
                .unwrap(),
            _ => unreachable!("Should not have unknown ops!!"),
        };

        if new_acc > target {
            continue;
        }

        let res = calculate_calibration_res(target, numbers, allowed_op, depth + 1, new_acc);
        if res == target {
            return target;
        }
    }

    0
}
fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let (target_str, numbers_str) = line.split_once(":").expect("Should find : in the input.");
    let target: usize = target_str.trim().parse::<usize>().unwrap();
    let numbers_vec: Vec<usize> = numbers_str
        .split(" ")
        .collect::<Vec<_>>()
        .iter()
        .filter(|str| !str.is_empty())
        .map(|str| str.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (target, numbers_vec)
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

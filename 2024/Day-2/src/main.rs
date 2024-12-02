use std::fs;

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let lines = utils::read_file_lines(INPUT);
    let mut safe_reports = 0;
    lines.iter().for_each(|line| {
        let report = parse_report(line);
        if is_report_safe(&report) {
            safe_reports += 1;
        }
    });
    println!("Number of safe reports: {}", safe_reports);
}

pub fn solve_part_2() {
    let lines = utils::read_file_lines(INPUT);
    let mut safe_reports = 0;
    lines.iter().for_each(|line| {
        let report = parse_report(line);
        if is_report_safe_with_dampener(&report) {
            safe_reports += 1;
        }
    });
    println!("Number of safe reports: {}", safe_reports);
}

fn parse_report(line: &str) -> Vec<u64> {
    let elements = line.split_whitespace().collect::<Vec<&str>>();
    elements
        .iter()
        .map(|number| number.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn is_report_safe(report: &Vec<u64>) -> bool {
    let mut is_increasing = true;
    for (i, val1) in report.iter().enumerate() {
        if i + 1 >= report.len() {
            break;
        }

        let val2 = &report[i + 1];
        //println!("Pair: ({}, {})", val1, val2);
        // second condition, 'Any two adjacent levels differ by at least
        // one and at most three.'
        let levels_diff = if val1 >= val2 {
            val1 - val2
        } else {
            val2 - val1
        };
        if levels_diff > 3 || levels_diff < 1 {
            return false;
        }

        // 'The levels are either all increasing or all decreasing.'
        if i == 0 {
            if val1 < val2 {
                is_increasing = true
            } else if val1 > val2 {
                is_increasing = false
            } else {
                return false;
            }
        }

        if (is_increasing && val1 > val2) || (!is_increasing && val1 < val2) {
            return false;
        }
    }
    true
}

fn is_report_safe_with_dampener(report: &Vec<u64>) -> bool {
    for (idx, _) in report.iter().enumerate() {
        let mut report_clone = report.clone();
        let removed_level = report_clone.remove(idx);
        if is_report_safe(&report_clone) {
            return true;
        }
    }

    false
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
    solve_part_1(); // 314
    solve_part_2(); // 373
}

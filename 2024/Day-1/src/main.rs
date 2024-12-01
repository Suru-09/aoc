use std::fs;

const INPUT: &str = "../input.txt";
const EXAMPLE: &str = "../example.txt";

mod part_1 {
    use crate::parse_line;
    use crate::utils::read_file_lines;
    use crate::INPUT;
    use std::iter::zip;
    use std::path::Path;

    pub fn solve_part_1() {
        let lines = read_file_lines(INPUT);
        let mut first_list = vec![];
        let mut second_list = vec![];
        lines.iter().for_each(|line| {
            let parsed_line = parse_line(line);
            first_list.push(parsed_line.0);
            second_list.push(parsed_line.1);
        });

        first_list.sort();
        second_list.sort();

        let mut distance: u64 = 0;
        for (a, b) in zip(first_list, second_list) {
            if a > b {
                distance += (a - b) as u64;
            } else {
                distance += (b - a) as u64;
            }
        }

        println!("Total distance is: {}", distance);
    }
}

mod part_2 {
    use crate::parse_line;
    use crate::utils::read_file_lines;
    use crate::INPUT;
    use std::collections::HashMap;
    use std::iter::zip;
    use std::path::Path;

    pub fn solve_part_2() {
        let lines = read_file_lines(INPUT);
        let mut first_list = vec![];
        let mut second_list = vec![];
        lines.iter().for_each(|line| {
            let parsed_line = parse_line(line);
            first_list.push(parsed_line.0);
            second_list.push(parsed_line.1);
        });

        first_list.sort();
        second_list.sort();

        let mut count_map = HashMap::new();

        first_list.iter().for_each(|first_number| {
            let appearances = second_list.iter().filter(|x| *x == first_number).count();
            match count_map.get(first_number) {
                Some(val) => {
                    count_map.insert(first_number, appearances as u64 + val);
                }
                None => {
                    count_map.insert(first_number, appearances as u64);
                }
            }
        });

        let mut result = 0;

        count_map.iter().for_each(|(key, value)| {
            result += *key * *value;
        });

        //println!("Count map : {:?}", count_map);
        println!("Result for part2 : {}", result);
    }
}

pub fn parse_line(line: &str) -> (u64, u64) {
    let elements = line.split_whitespace().collect::<Vec<&str>>();
    assert!(elements.len() == 2);
    let first_integer = elements
        .first()
        .expect("Just checked len..")
        .trim()
        .parse::<u64>()
        .unwrap();
    let second_integer = elements
        .last()
        .expect("Just checked len..")
        .trim()
        .parse::<u64>()
        .unwrap();
    (first_integer, second_integer)
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
    part_1::solve_part_1(); // 2000468
    part_2::solve_part_2(); // 18567089
}
